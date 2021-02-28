# Google Authenticator Extractor

[![Build](https://github.com/zhangyuan/google-authenticator-extractor/workflows/Build/badge.svg)](https://github.com/zhangyuan/google-authenticator-extractor/actions/workflows/build.yml)
## What and Why

This project aims at providing a single executable program to extract the OTP accounts from the QR code image exported from Google Authenticator, inspired by [extract_otp_secret_keys](https://github.com/scito/extract_otp_secret_keys) written in Python.

## How to use it

### Get an image of the QR code

1. You can open Google Authenticator, tap on the three dots on the top right of the screen and select “Transfer accounts”, and follow the instructions to get the QR code image.
2. Move the QR code to your laptop. An easy and secure way on Mac OSX is using Photo Booth to take a photo of the QR code and save it to your laptop.

### Run the program to extract the accounts

Download the execuable binary from [Releases](https://github.com/zhangyuan/google-authenticator-extractor/releases). Currently the executable binary is built for OSX only.

```bash
chmod a+x google_authenticator_extractor-x86_64-apple-darwin
./google_authenticator_extractor-x86_64-apple-darwin -i /path/to/qrcode-image
```

> Note: OSX may prevent it from running with the message `“google_authenticator_extractor-x86_64-apple-darwin” cannot be opened because the developer cannot be verified.`. You need to go to `System Preferences` - `Security & Privacy` - `General` to allow it.

The output looks like as below:

```json
[{"name":"******","secret":"**********"},{"name":"*****","secret":"************"}]
```

## How to build from source

> Rust is the programming language of the project, Install Rust by following the instructions in the [official doument](https://www.rust-lang.org/tools/install).

### On Mac OS X

Install protobuf with Homebrew:

```bash
brew install protobuf
```

Build the project

```bash
./build.sh
```

### On Ubuntu

Install protobuf with `apt-get`:

```bash
sudo apt-get update
sudo apt-get install -y protobuf-compiler
```

Build the project:

```bash
./build.sh
```

### With Docker

Build the docker image with the `Dockerfile`:

```bash
docker build . -t google_authenticator_extractor
```

Run the docker image and with command `bash`, then you'll be in the docker container.

```
docker run -v ./:/app --rm -it google_authenticator_extractor bash
```

### Cross Compilation

The Github actions defined steps cross compilation, which are running on CI as well. Currently the following cross compilations are supported.

* Build for `x86_64-pc-windows-gnu` on Mac OSX.
* Build for `x86_64-unknown-linux-gnu` on Mac OSX.
* Build for `x86_64-pc-windows-gnuu` on Ubuntu.

Check the [Github Workflow](./github/workflows/build.yml) for more details.