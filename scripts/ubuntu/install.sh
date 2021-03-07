
#! /bin/bash

set -eou pipefail

rustup component add clippy

sudo apt-get update
sudo apt-get install -y protobuf-compiler build-essential mingw-w64

rustup toolchain install stable-x86_64-pc-windows-gnu
rustup target add x86_64-pc-windows-gnu

rustup toolchain install stable-x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu