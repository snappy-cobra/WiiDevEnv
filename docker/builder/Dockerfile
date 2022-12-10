FROM ubuntu:22.04

# Install Wii Dev environment
WORKDIR /
RUN apt-get update
RUN apt-get install -y sudo wget inotify-tools unzip build-essential clang libclang-dev dosfstools
COPY install-devkitpro-pacman.sh /install-devkitpro-pacman.sh
RUN chmod +x ./install-devkitpro-pacman.sh
RUN sudo ./install-devkitpro-pacman.sh
RUN if [ ! -f /etc/mtab ]; then sudo ln -s /proc/self/mounts /etc/mtab; fi;
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
RUN mkdir /app
RUN mkdir /app/src
RUN mkdir /app/data
RUN mkdir /build
RUN mkdir /build/target
RUN mkdir /build/bin

# Make sure the target is set correctly.
ENV CARGO_TARGET_DIR="/build/target"

# Install Rust Wii Dev environment
COPY install_rust.sh /install_rust.sh
RUN chmod +x /install_rust.sh
RUN /install_rust.sh

# To ease docker build caching: add remaining files.
COPY build_watch.sh /build_watch.sh
RUN chmod +x /build_watch.sh

# Go to the project and start the main script
CMD ["/build_watch.sh"]
