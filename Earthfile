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

# headless-dolphin:
#   FROM ubuntu:bionic
#   RUN apt update
#   RUN apt install -y software-properties-common gpg xvfb xdg-utils alsa-utils
#   RUN apt-add-repository ppa:dolphin-emu/ppa
#   RUN apt update
#   RUN apt install -y dolphin-emu-master
#   RUN mkdir /root/.config

headless-dolphin:
  FROM debian:bullseye-slim
  RUN apt update \
   && apt install -y --no-install-recommends xvfb xauth alsa-utils git ca-certificates qtbase5-dev qtbase5-private-dev git cmake make gcc g++ pkg-config libavcodec-dev libavformat-dev libavutil-dev libswscale-dev libxi-dev libxrandr-dev libudev-dev libevdev-dev libsfml-dev libminiupnpc-dev libmbedtls-dev libcurl4-openssl-dev libhidapi-dev libsystemd-dev libbluetooth-dev libasound2-dev libpulse-dev libpugixml-dev libbz2-dev libzstd-dev liblzo2-dev libpng-dev libusb-1.0-0-dev gettext \
    && apt autoremove
  RUN git clone https://github.com/dolphin-emu/dolphin.git ./dolphin-emu
  WORKDIR ./dolphin-emu
  RUN git submodule update --init --recursive
  RUN mkdir Build && cd Build && cmake .. && make -j$(nproc) && make install


integration-test:
  FROM +headless-dolphin
  RUN mkdir /build
  COPY +build-integration-test/rust-wii.elf /build/rust-wii.elf
  # ENV QT_QPA_PLATFORM=linuxfb # Disable rendering
  # RUN modprobe snd-dummy # Enable dummy sound driver
  # RUN xvfb-run --server-args="-screen 0, 1920x1080x24" \
  #     dolphin-emu -v=null --exec=/build/rust-wii.elf
      RUN dolphin-emu -v=null --exec=/build/rust-wii.elf
  # RUN xvfb-run \
  #       \
  #     which dolphin-emu-master --version # --batch --exec=/wii/rust-wii.elf


# Run all tests and sanity checks
test:
  BUILD +build # Normal compilation should work without problems
  BUILD +unit-test # Unit test suite
  BUILD +integration-test
  # TODO Clippy?
