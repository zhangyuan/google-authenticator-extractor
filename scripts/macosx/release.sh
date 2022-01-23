#! /bin/bash

set -e

echo "Build for default target"
./scripts/build.sh

echo "Build for x86_64-pc-windows-gnu"
./scripts/build.sh x86_64-pc-windows-gnu

echo "Build for x86_64-unknown-linux-gnu"
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc ./scripts/build.sh x86_64-unknown-linux-gnu

mv target/release/google_authenticator_extractor \
  releases/google_authenticator_extractor-x86_64-apple-darwin

if [[ "$OSTYPE" == "darwin"* ]]; then
  upx releases/google_authenticator_extractor-x86_64-apple-darwin
fi


mv target/x86_64-pc-windows-gnu/release/google_authenticator_extractor.exe \
  releases/google_authenticator_extractor-x86_64-pc-windows-gnu.exe

mv target/x86_64-unknown-linux-gnu/release/google_authenticator_extractor \
  releases/google_authenticator_extractor-x86_64-unknown-linux-gnu
