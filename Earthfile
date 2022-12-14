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
  RUN cargo +nightly build -Z build-std=core,alloc --target powerpc-unknown-eabi.json
  SAVE ARTIFACT /build/target/powerpc-unknown-eabi/debug/rust-wii.elf AS LOCAL /build/bin/boot.elf

build-integration-test:
  FROM +build-env
  COPY ./app/ /app/
  WORKDIR /app/
  RUN cargo +nightly build --features=run_target_tests -Z build-std=core,alloc --target powerpc-unknown-eabi.json
  SAVE ARTIFACT /build/target/powerpc-unknown-eabi/debug/rust-wii.elf AS LOCAL /build/bin/boot-test.elf

test:
  FROM +build-env
  WORKDIR app/
  COPY ./app/ ./
  WORKDIR app/lib/
  RUN cargo test

unit-test:
  FROM ghcr.io/rust-lang/rust:nightly-slim
  COPY ./app/ /app/
  WORKDIR /app/lib/
  RUN cargo test -Zbuild-std
