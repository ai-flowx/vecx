#!/bin/bash

type="$1"
version="$2"

build=$(date +%FT%T%z)
commit=$(git rev-parse --short HEAD)

target_linux="target/x86_64-unknown-linux-gnu/release/vecx"
target_windows="target/x86_64-pc-windows-gnu/release/vecx.exe"

if [ "$type" = "all" ]; then
  build=$build cargo build --release --target=x86_64-apple-darwin --target=aarch64-apple-darwin
  build=$build cargo build --release --target=x86_64-pc-windows-gnu
  build=$build cargo build --release --target=x86_64-unknown-linux-gnu
elif [ "$type" = "offline" ]; then
  build=$build cargo build --release --target=x86_64-apple-darwin --target=aarch64-apple-darwin --offline
  build=$build cargo build --release --target=x86_64-pc-windows-gnu --offline
  build=$build cargo build --release --target=x86_64-unknown-linux-gnu --offline
elif [ "$type" = "check" ]; then
  build=$build cargo check --release --all-features --all-targets
else
  build=$build cargo build --release --target=x86_64-unknown-linux-gnu
fi

if [ -f "${target_linux}" ]; then upx "${target_linux}"; fi
if [ -f "${target_windows}" ]; then upx "${target_windows}"; fi
