VERSION 0.6
FROM ghcr.io/rust-lang/rust:nightly-slim

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

build:
  FROM +build-env
  COPY ./app/ /app/
  WORKDIR /app/
  RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo +nightly build -Z build-std=core,alloc --target powerpc-unknown-eabi.json
  SAVE ARTIFACT /build/target/powerpc-unknown-eabi/debug/rust-wii.elf AS LOCAL /build/bin/boot.elf

build-integration-test:
  FROM +build-env
  COPY ./app/ /app/
  WORKDIR /app/
  RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo +nightly build --features=run_target_tests -Z build-std=core,alloc --target powerpc-unknown-eabi.json
  SAVE ARTIFACT /build/target/powerpc-unknown-eabi/debug/rust-wii.elf AS LOCAL /build/bin/boot-test.elf

unit-test:
  FROM ghcr.io/rust-lang/rust:nightly-slim
  RUN rustup +nightly component add rust-src
  COPY ./app/lib/ /app/lib/
  WORKDIR /app/lib/
  RUN --mount=type=cache,target=/usr/local/cargo/registry/index \
      --mount=type=cache,target=/usr/local/cargo/registry/cache \
      --mount=type=cache,target=/usr/local/cargo/git/db \
      cargo +nightly test


# Tiny Docker image only containing the Dolphin emulator
# Based on https://github.com/rmzi/dolphin-docker/
# WIP
dolphin-docker:
  FROM debian:slim
  RUN apt-get update \
  && apt-get install dolphin-emu \
  && apt-get autoremove \
  && rm -rf /var/lib/apt/lists/* \

# TODO
integration-test:
  FROM +dolphin-docker
  RUN mkdir /build/bin
  COPY +build-integration-test/rust-wii.elf /build/bin/rust-wii.elf
  RUN timeout 5s /usr/games/dolphin-emu --batch --exec=/build/bin/rust-wii.elf

test:
  BUILD +build # Normal compilation should work without problems
  BUILD +unit-test # Unit test suite
  BUILD +integration-test
  # TODO Clippy?
