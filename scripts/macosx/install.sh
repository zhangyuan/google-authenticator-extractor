#! /bin/bash

set -eo pipefail

# for target x86_64-pc-windows-gnu
brew install mingw-w64
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup target add x86_64-pc-windows-gnu

# for target x86_64-unknown-linux-gnu
brew tap SergioBenitez/osxct
brew install x86_64-unknown-linux-gnu
rustup toolchain install stable-x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu

# to compress executables
brew install upx
