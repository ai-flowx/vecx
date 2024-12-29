#!/bin/bash

# Install for gcc-mingw
sudo apt install --yes --no-install-recommends gcc-mingw-w64-x86-64-win32

# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Update for stable
rustup update --no-self-update stable

# Install for coverage
cargo install cargo-tarpaulin

# Install for lint
cargo install cargo-hack --debug
rustup component add clippy
rustup component add rustfmt

# Install for upx
sudo apt update -y
sudo apt install -y upx

# Install for Windows
# rustc --print target-list
sudo apt update -y
sudo apt install -y mingw-w64
rustup target add x86_64-pc-windows-gnu
rustup toolchain install --force-non-host stable-x86_64-pc-windows-gnu

# Install for Linux
# rustc --print target-list
rustup target add x86_64-unknown-linux-gnu
rustup toolchain install --force-non-host stable-x86_64-unknown-linux-gnu

# Install for macOS
# rustc --print target-list
rustup target add x86_64-apple-darwin aarch64-apple-darwin
rustup toolchain install --force-non-host stable-x86_64-apple-darwin
rustup toolchain install --force-non-host stable-aarch64-apple-darwin
