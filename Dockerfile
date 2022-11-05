FROM ubuntu:22.04

# Install Wii Dev environment
WORKDIR /
RUN apt-get update && apt-get upgrade -y --fix-missing
RUN apt-get install -y sudo wget inotify-tools unzip build-essential clang libclang-dev
COPY install-devkitpro-pacman install-devkitpro-pacman
RUN chmod +x ./install-devkitpro-pacman
RUN sudo ./install-devkitpro-pacman
RUN sudo dkp-pacman --noconfirm -S wii-dev

ENV DEVKITPRO=/opt/devkitpro
ENV DEVKITARM="${DEVKITPRO}/devkitARM"
ENV DEVKITPPC="${DEVKITPRO}/devkitPPC"
ENV PATH="${PATH}:${DEVKITPPC}/bin/"

# Install Wii 3D Dev lib: GRRLIB
RUN curl -L https://github.com/GRRLIB/GRRLIB/archive/master.zip > GRRLIB.zip
RUN unzip GRRLIB.zip && rm GRRLIB.zip
WORKDIR /GRRLIB-master/GRRLIB/
RUN sudo dkp-pacman --sync --needed --noconfirm libfat-ogc ppc-libpng ppc-freetype ppc-libjpeg-turbo
RUN make clean all install
WORKDIR /

# Setup build folder structure
RUN mkdir /project
RUN mkdir /project/target
RUN mkdir /project/bin
RUN mkdir /project/build
RUN mkdir /project/src
RUN mkdir /project/data

# Install Rust Wii Dev environment
COPY Cargo.toml /project/Cargo.toml
COPY install_rust /project/install_rust
RUN chmod +x /project/install_rust
WORKDIR /project
RUN /project/install_rust

# To ease docker build caching: add remaining files.
COPY .cargo /project/.cargo
COPY powerpc-unknown-eabi.json /project/powerpc-unknown-eabi.json
COPY wrapper.h /project/wrapper.h
COPY build.rs /project/build.rs
COPY build_watch /project/build_watch
RUN chmod +x /project/build_watch

# Go to the project and start the main script
CMD ["./build_watch"]
