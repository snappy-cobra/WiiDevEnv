#!/usr/bin/env bash

# Set the CWD to the app folder
cd /app

# Build function
build () {
    echo -e "\e[1;34m Running Rust formatter... \e[0m"
    cargo +nightly fmt
    echo -e "\e[1;34m Showing Clippy warnings... \e[0m"
    cargo +nightly clippy --target=powerpc-unknown-eabi.json --color=always

    echo -e "\e[1;34m Starting main build... \e[0m"
    cargo +nightly build -Z build-std=core,alloc --target=powerpc-unknown-eabi.json --color=always
    cp /build/target/powerpc-unknown-eabi/debug/rust-wii.elf /build/bin/boot.elf
    echo -e "\e[1;32m Binary main build completed. \e[0m"
}

# Run script
echo -e "\e[1;34m Starting initial build... \e[0m"
build
echo -e "\e[1;34m Watch started. \e[0m"
inotifywait -mq -r -e create -e modify -e delete -e move ./src ./Cargo.toml ./powerpc-unknown-eabi.json ./gamelib ./modulator ./grrustlib |
    while read dir action file; do
        echo -e "\e[1;34m The file '$file' appeared in directory '$dir' via '$action', rebuilding... \e[0m"
        build
    done
