#! /bin/bash

set -eo pipefail

TARGETS="$@"

cargo clippy -- -D warnings

if [ -z "${TARGETS}" ]; then
  echo "Build for the host system"
  cargo build --release
fi

for TARGET in ${TARGETS}
do
  echo "Build for '${TARGET}'"
  cargo build --release --target "${TARGET}"
done
