#! /bin/bash

set -eou pipefail

cargo clean

cargo build --release
