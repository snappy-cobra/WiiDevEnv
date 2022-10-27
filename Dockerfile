FROM ubuntu:22.04

# Install C++ Wii Dev environment
WORKDIR /
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y sudo wget inotify-tools unzip build-essential
COPY install-devkitpro-pacman install-devkitpro-pacman
RUN chmod +x ./install-devkitpro-pacman
RUN sudo ./install-devkitpro-pacman
RUN sudo dkp-pacman --noconfirm -S wii-dev

ENV DEVKITPRO=/opt/devkitpro
ENV DEVKITPPC=/opt/devkitpro/devkitPPC

# Install Wii 3D Dev lib: GRRLIB
RUN curl -L https://github.com/GRRLIB/GRRLIB/archive/master.zip > GRRLIB.zip
RUN unzip GRRLIB.zip && rm GRRLIB.zip
WORKDIR /GRRLIB-master/GRRLIB/
RUN sudo dkp-pacman --sync --needed --noconfirm libfat-ogc ppc-libpng ppc-freetype ppc-libjpeg-turbo
RUN make clean all install

# Install Rust Wii Dev environment
COPY install_rust install_rust
RUN chmod +x install_rust
RUN ./install_rust

# Setup build folder structure and required files
RUN mkdir /project
RUN mkdir /project/bin
RUN mkdir /project/target
RUN mkdir /project/build
RUN mkdir /project/source
RUN mkdir /project/data

COPY powerpc-unknown-eabi.json /project/powerpc-unknown-eabi.json
copy Cargo.toml /project/Cargo.toml
COPY Makefile /project/Makefile
COPY build_watch /project/build_watch
RUN chmod +x /project/build_watch

# Go to the project and start the main script
WORKDIR /project
CMD ["./build_watch"]
