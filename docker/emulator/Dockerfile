FROM ubuntu:20.04

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update
RUN apt-get install -y dolphin-emu

# add dolphin-emu bin to path
RUN ln -s /usr/games/dolphin-emu /usr/local/bin/dolphin-emu

# Copy the kickstart script
COPY start_emulator.sh /start_emulator.sh
RUN chmod +x /start_emulator.sh

# Start the main script
CMD ["/start_emulator.sh"]
