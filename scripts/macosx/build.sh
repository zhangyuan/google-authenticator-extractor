#! /bin/bash

set -e

echo "Build for default target"
./scripts/build.sh

echo "Build for x86_64-pc-windows-gnu"
./scripts/build.sh x86_64-pc-windows-gnu

echo "Build for x86_64-unknown-linux-gnu"
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc ./scripts/build.sh x86_64-unknown-linux-gnu