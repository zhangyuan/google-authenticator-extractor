#! /bin/bash

set -e

echo "Build for default target"
./scripts/build.sh

echo "Build for x86_64-pc-windows-gnu"
./scripts/build.sh x86_64-pc-windows-gnu

echo "Build for x86_64-unknown-linux-gnu"
./scripts/build.sh x86_64-unknown-linux-gnu