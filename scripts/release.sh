#! /bin/bash

set -e

mv target/release/google_authenticator_extractor releases/google_authenticator_extractor-x86_64-apple-darwin
mv target/x86_64-pc-windows-gnu/release/google_authenticator_extractor.exe releases/google_authenticator_x86_64-pc-windows-gnu.exe
mv target/x86_64-unknown-linux-gnu/release/google_authenticator_extractor releases/google_authenticator_x86_64-unknown-linux-gnu