
#! /bin/bash

set -eou pipefail

apt-get update
apt-get install -y build-essential mingw-w64

rustup toolchain install stable-x86_64-pc-windows-gnu
rustup target add x86_64-pc-windows-gnu

rustup toolchain install stable-x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu

