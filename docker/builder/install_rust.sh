#!/usr/bin/env bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup default nightly
rustup component add rust-src --toolchain nightly
rustup component add clippy --toolchain nightly
rustup component add rustfmt --toolchain nightly
