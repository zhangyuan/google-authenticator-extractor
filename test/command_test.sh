#! /bin/bash

set -eo pipefail

set -eo pipefail

IMAGE="test/fixtures/migration.jpg"

actual=$(./target/release/google_authenticator_extractor -i ${IMAGE})

expected='[{"name":"Example:example@example.local","secret":"43AHZP6DT7U4CM5PAJRT652BBEHXXPT72XJVFBISFVEZ3MFB33MQBCPR","issuer":""}]'

if [ "${actual}" == "${expected}" ]; then
  echo "Test passed."
else
  echo "Actual: ${actual}"
  echo "Expected: ${expected}"
  echo "Test failed."
  exit 1
fi