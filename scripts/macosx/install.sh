#! /bin/bash

set -eo pipefail

brew install protobuf

# for target x86_64-pc-windows-gnu
brew install mingw-w64

# for target x86_64-unknown-linux-gnu
brew tap SergioBenitez/osxct
brew install x86_64-unknown-linux-gnu