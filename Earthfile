VERSION --use-cache-command 0.6

# Portable build environment, containing
# - Rust
# - DevkitPro + its wii-dev package
# - Grrlib
build-env:
  FROM ghcr.io/rust-lang/rust:nightly-slim
  CACHE /usr/local/cargo/registry/index
  CACHE /usr/local/cargo/registry/cache
  CACHE /usr/local/cargo/git/db
  WORKDIR /
  COPY ./docker/builder/install-devkitpro-pacman.sh /install-devkitpro-pacman.sh
  RUN chmod +x ./install-devkitpro-pacman.sh
  RUN apt-get update && \
    apt-get install -y git sudo wget inotify-tools unzip build-essential clang libclang-dev dosfstools && \
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
  SAVE IMAGE --push=qqwy/wii-rust-build-env:latest

build-env-all-platforms:
  BUILD --platform=linux/arm64 --platform=linux/amd64 +build-env

dolphin-all-platforms:
  BUILD --platform=linux/arm64 --platform=linux/amd64 +dolphin


build-deps:
  FROM +rust-cargo-chef
  WORKDIR /app/
  COPY ./app/Cargo.* ./app/powerpc-unknown-eabi.json ./
  RUN cargo +nightly chef prepare --recipe-path recipe.json
  SAVE ARTIFACT recipe.json
  SAVE IMAGE --cache-hint

# Build the main game Wii ROM
build-prepare:
  # FROM +build-env
  FROM +rust-cargo-chef

  # Build only lib/ dependencies, cacheable:
  WORKDIR /app/lib/
  COPY +unit-test-deps/recipe.json ./
  COPY ./app/powerpc-unknown-eabi.json ./
  RUN cargo +nightly chef cook --no-std --recipe-path recipe.json --features=wii -Z build-std=core,alloc --target powerpc-unknown-eabi.json
  SAVE IMAGE --cache-hint

  # Only copy the rest of /app/lib afterwards:
  COPY ./app/lib/ ./
  SAVE IMAGE --cache-hint

  WORKDIR /app/

  # Build only dependencies, cacheable:
  COPY ./app/powerpc-unknown-eabi.json ./
  COPY +build-deps/recipe.json ./
  RUN cargo +nightly chef cook --no-std --recipe-path recipe.json -Z build-std=core,alloc --target powerpc-unknown-eabi.json
  SAVE IMAGE --cache-hint

  COPY ./app/ .
  SAVE IMAGE --cache-hint

build:
  FROM +build-prepare
  RUN cargo +nightly build -Z build-std=core,alloc --target powerpc-unknown-eabi.json
  SAVE ARTIFACT /build/target/powerpc-unknown-eabi/debug/rust-wii.elf AS LOCAL ./bin/boot.elf
  SAVE ARTIFACT ./Cargo.lock AS LOCAL ./app/Cargo.lock
  SAVE IMAGE --cache-hint

# Build a Wii ROM that runs the on-target-device integration test suite.
build-integration-test:
  FROM +build-prepare
  RUN cargo +nightly build --features=run_target_tests -Z build-std=core,alloc --target powerpc-unknown-eabi.json
  SAVE ARTIFACT /build/target/powerpc-unknown-eabi/debug/rust-wii.elf AS LOCAL ./bin/boot-test.elf
  SAVE ARTIFACT ./Cargo.lock AS LOCAL ./app/Cargo.lock
  SAVE IMAGE --cache-hint

rust-cargo-chef:
  FROM qqwy/wii-rust-build-env
  CACHE /usr/local/cargo/registry/
  CACHE /usr/local/cargo/git/
  CACHE /build/target
  RUN cargo install --git=https://github.com/Qqwy/cargo-chef.git --branch=trim_target_suffix
  RUN rustup +nightly component add rust-src
  SAVE IMAGE --cache-hint

unit-test-deps:
  FROM +rust-cargo-chef
  WORKDIR /app/lib/
  COPY ./app/lib/Cargo.* ./
  RUN cargo +nightly chef prepare  --recipe-path recipe.json
  SAVE ARTIFACT recipe.json
  SAVE IMAGE --cache-hint

# Run unit tests of the `app/lib` subcrate using the normal Rust test flow.
unit-test:
  FROM +rust-cargo-chef
  # Build only dependencies, cacheable:
  WORKDIR /app/lib/
  COPY +unit-test-deps/recipe.json ./
  RUN cargo +nightly chef cook --recipe-path recipe.json
  SAVE IMAGE --cache-hint

  # Build and test app:
  COPY ./app/lib/ ./
  RUN cargo +nightly test --color=always
  SAVE ARTIFACT ./Cargo.lock AS LOCAL ./app/lib/Cargo.lock

# BASE IMAGE CONTAINING DOLPHIN
# -----------------------------
dolphin:
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

  # Download, initialize and build the newest dev version of Dolphin:
  # And clean up afterwards (all in the same layer!) to keep Docker image smaller.
  RUN git clone https://github.com/dolphin-emu/dolphin.git ./dolphin-emu && \
      cd ./dolphin-emu && \
      git submodule update --init --recursive && \
      mkdir Build && cd Build && cmake .. && make -j$(nproc) && make install && cd ../ && \
      cd ../ && \
      rm -rf ./dolphin-emu

  SAVE IMAGE --push=qqwy/dolphin-emu:latest

# IMAGE RUNNING THE ROM ON DOLPHIN
# Actually running the ROM is kept as CMD
# This is necessary because we need to specify
# a custom large `--shm-size` to `docker run`
# if we do not want Dolphin to crash.
# --------------------------------
integration-test-runner:
  # For speed in CI, we use a prior built image rather than depending on the target from within this Earthfile
  # FROM +dolphin
  # FROM --platform=linux/amd64 ghcr.io/qqwy/dolphin:latest
  FROM qqwy/dolphin:latest


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
  # # timeout 5s: Dolphin hangs on panic. This converts a hang to a non-zero exit code.
  # # dolphin-emu: Run Dolphin
  # # 2>&1: Redirect stderr (which Dolphin logs to) to stdout
  # # grep: Look in the log output only for lines containing 'OSREPORT_HLE' as those are where print statements and panics end up.
  CMD xvfb-run \
      timeout --signal=KILL 1m \
      dolphin-emu --batch --exec=/build/boot.elf \
      2>&1
  SAVE IMAGE itr integration-test-runner:latest


integration-test:
  LOCALLY
  WITH DOCKER --load="integration-test-runner:latest=+integration-test-runner"
    RUN docker run --shm-size=4G integration-test-runner:latest
  END

# Run all tests and sanity checks
test:
  # BUILD +build # Normal compilation should work without problems
  BUILD +unit-test # Unit test suite
  BUILD +integration-test
  # TODO Clippy?


watch:
  LOCALLY
  RUN fswatch --one-per-batch --recursive ./app/lib ./app/modulator ./app/src ./app/data ./app/Cargo.toml ./app/build.rs ./app/wrapper.h ./app/powerpc-unknown-eabi.json | \
    while read dir action file; do \
      echo -e "\e[1;34m The file '$file' appeared in directory '$dir' via '$action', rebuilding and retesting... \e[0m"; \
      FORCE_COLOR=1 earthly +build && \
      FORCE_COLOR=1 earthly +test; \
    done

