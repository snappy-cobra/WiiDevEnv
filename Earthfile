VERSION 0.6
FROM ghcr.io/rust-lang/rust:nightly-slim

# Portable build environment, containing
# - Rust
# - DevkitPro + its wii-dev package
# - Grrlib
build-env:
  WORKDIR /
  COPY ./docker/builder/install-devkitpro-pacman.sh /install-devkitpro-pacman.sh
  RUN chmod +x ./install-devkitpro-pacman.sh
  RUN apt-get update && \
    apt-get install -y sudo wget inotify-tools unzip build-essential clang libclang-dev dosfstools && \
    sudo ./install-devkitpro-pacman.sh && \
    apt-get purge -y && \
    rm -rf /var/lib/apt/lists/*
  RUN if [ ! -f /etc/mtab ]; then sudo ln -s /proc/self/mounts /etc/mtab; fi;
  RUN sudo dkp-pacman --noconfirm -S wii-dev

  ENV DEVKITPRO=/opt/devkitpro
  ENV DEVKITARM="${DEVKITPRO}/devkitARM"
  ENV DEVKITPPC="${DEVKITPRO}/devkitPPC"
  ENV PATH="${PATH}:${DEVKITPPC}/bin/"

  # Install Wii 3D Dev lib: GRRLIB
  RUN curl -L https://github.com/GRRLIB/GRRLIB/archive/master.zip > GRRLIB.zip && unzip GRRLIB.zip && rm GRRLIB.zip
  WORKDIR /GRRLIB-master/GRRLIB/
  RUN sudo dkp-pacman --sync --needed --noconfirm libfat-ogc ppc-libpng ppc-freetype ppc-libjpeg-turbo
  RUN make clean all install
  WORKDIR /

  # Setup build folder structure
  RUN mkdir /app
  RUN mkdir /app/src
  RUN mkdir /app/data
  RUN mkdir /build
  RUN mkdir /build/target
  RUN mkdir /build/bin

  # Make sure the target is set correctly.
  ENV CARGO_TARGET_DIR="/build/target"
  RUN rustup component add rust-src --toolchain nightly

# Build the main game Wii ROM
build:
  FROM +build-env
  COPY ./app/ /app/
  WORKDIR /app/
  RUN --mount=type=cache,target=/usr/local/cargo/registry/index \
      --mount=type=cache,target=/usr/local/cargo/registry/cache \
      --mount=type=cache,target=/usr/local/cargo/git/db \
      cargo +nightly build -Z build-std=core,alloc --target powerpc-unknown-eabi.json
  SAVE ARTIFACT /build/target/powerpc-unknown-eabi/debug/rust-wii.elf AS LOCAL ./bin/boot.elf

# Build a Wii ROM that runs the on-target-device integration test suite.
build-integration-test:
  FROM +build-env
  COPY ./app/ /app/
  WORKDIR /app/
  RUN --mount=type=cache,target=/usr/local/cargo/registry/index \
      --mount=type=cache,target=/usr/local/cargo/registry/cache \
      --mount=type=cache,target=/usr/local/cargo/git/db \
      cargo +nightly build --features=run_target_tests -Z build-std=core,alloc --target powerpc-unknown-eabi.json
  SAVE ARTIFACT /build/target/powerpc-unknown-eabi/debug/rust-wii.elf AS LOCAL ./bin/boot-test.elf

# Run unit tests of the `app/lib` subcrate using the normal Rust test flow.
unit-test:
  FROM ghcr.io/rust-lang/rust:nightly-slim
  RUN rustup +nightly component add rust-src
  COPY ./app/lib/ /app/lib/
  WORKDIR /app/lib/
  RUN --mount=type=cache,target=/usr/local/cargo/registry/index \
      --mount=type=cache,target=/usr/local/cargo/registry/cache \
      --mount=type=cache,target=/usr/local/cargo/git/db \
      cargo +nightly test --color=always

# BASE IMAGE CONTAINING DOLPHIN
# -----------------------------
headless-dolphin:
  FROM debian:bullseye-slim

  # Install dependencies for building Dolphin
  # As well as `xvfb` and `xauth` to fake a display
  # `xdg-utils` to fix a warning related to mime-types
  # and `alsa-utils` to fake sound drivers
  RUN apt update \
  && apt install -y --no-install-recommends \
  xvfb xauth xdg-utils alsa-utils \
  git ca-certificates qtbase5-dev qtbase5-private-dev git cmake make gcc g++ pkg-config libavcodec-dev libavformat-dev libavutil-dev libswscale-dev libxi-dev libxrandr-dev libudev-dev libevdev-dev libsfml-dev libminiupnpc-dev libmbedtls-dev libcurl4-openssl-dev libhidapi-dev libsystemd-dev libbluetooth-dev libasound2-dev libpulse-dev libpugixml-dev libbz2-dev libzstd-dev liblzo2-dev libpng-dev libusb-1.0-0-dev gettext \
  && apt autoremove \
  && apt purge -y \
  && rm -rf /var/lib/apt/lists/*

  # Download + initialize the newest dev version of Dolphin:
  RUN git clone https://github.com/dolphin-emu/dolphin.git ./dolphin-emu
  WORKDIR ./dolphin-emu
  RUN git submodule update --init --recursive

  # Build Dolphin:
  RUN mkdir Build && cd Build && cmake .. && make -j$(nproc) && make install

# IMAGE RUNNING THE ROM ON DOLPHIN
# Actually running the ROM is kept as CMD
# This is necessary because we need to specify
# a custom large `--shm-size` to `docker run`
# if we do not want Dolphin to crash.
# --------------------------------
integration-test-runner:
  FROM +headless-dolphin

  # Copy ROM into image:
  RUN mkdir /build
  COPY +build-integration-test/rust-wii.elf /build/boot.elf

  # Run rest of the commands as unprivileged user:
  RUN adduser --disabled-password --gecos '' user
  USER user

  # Configuration to redirect sound output to the 'null' sound card:
  RUN echo "pcm.!default null\nctl.!default null\n" > ~/.asoundrc

  # Dolphin configuration settings:
  # RUN mkdir ~/.dolphin-emu
  COPY --chown=user:user ./dolphin-emu /home/user/.dolphin-emu
  WORKDIR /home/user/

  # Run Dolphin using a fake display:
  # This command should work fine but crashes with a 'Bus error'.
  # RUN xvfb-run dolphin-emu --batch --exec=/build/boot.elf
  # CMD xvfb-run dolphin-emu --exec=/build/boot.elf --batch
  # Alternative version that hangs without logging output ever appearing:
  # RUN QT_QPA_PLATFORM=offscreen dolphin-emu --exec=/build/boot.elf

  # # Desired command we really would like to run.
  # # Explanation:
  # # xvfb-run: With a fake display
  # # timeout 5s: Dolphin hangs on panic. This converts a hang to a non-zero exit code
  # # dolphin-emu-nogui: Run Dolphin
  # # 2>&1: Redirect stderr (which Dolphin logs to) to stdout
  # # grep: Look in the log output only for lines containing 'OSREPORT_HLE' as those are where print statements and panics end up.
  CMD xvfb-run \
      timeout 5s \
      dolphin-emu-nogui --platform=headless --exec=/build/boot.elf \
      2>&1 | grep "OSREPORT_HLE"
  SAVE IMAGE itr


integration-test:
  FROM earthly/dind:alpine
  WITH DOCKER --load=+integration-test-runner
    RUN docker run --shm-size=8G itr
  END

# Run all tests and sanity checks
test:
  BUILD +build # Normal compilation should work without problems
  BUILD +unit-test # Unit test suite
  BUILD +integration-test
  # TODO Clippy?
