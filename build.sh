#! /bin/bash

set -eou pipefail

set -x

rustup target add x86_64-apple-darwin #--toolchain stable
cargo build --target x86_64-apple-darwin

rustup target add x86_64-unknown-linux-gnu #--toolchain stable
cargo build --target x86_64-unknown-linux-gnu