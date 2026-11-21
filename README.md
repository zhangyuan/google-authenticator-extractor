# Google Authenticator Extractor

[![Build](https://github.com/zhangyuan/google-authenticator-extractor/workflows/Build/badge.svg)](https://github.com/zhangyuan/google-authenticator-extractor/actions/workflows/build.yml)
## What and Why

This project aims at providing a single executable program to extract the OTP accounts from the QR code image exported from Google Authenticator, inspired by [extract_otp_secret_keys](https://github.com/scito/extract_otp_secret_keys) written in Python.

## How to use it

### Get an image of the QR code

1. You can open Google Authenticator, tap on the three dots on the top right of the screen and select “Transfer accounts”, and follow the instructions to get the QR code image.
2. Move the QR code to your laptop. An easy and secure way on Mac OSX is using Photo Booth to take a photo of the QR code and save it to your laptop.

### Run the program to extract the accounts

Download the execuable binary from [Releases](https://github.com/zhangyuan/google-authenticator-extractor/releases), and run the executable binary with `-i` option.

For example, download the executable binary `google_authenticator_extractor-x86_64-apple-darwin` for Mac OSX, and run the following command in terminal:

```bash
chmod a+x google_authenticator_extractor-x86_64-apple-darwin
./google_authenticator_extractor-x86_64-apple-darwin -i /path/to/qrcode-image
```

If Google Authenticator split the export across several QR codes, pass all of
them at once. You can repeat `-i`, list several files after one `-i`, or point
`-i` at a directory:

```bash
./google_authenticator_extractor-x86_64-apple-darwin -i qr-1.png qr-2.png qr-3.png
./google_authenticator_extractor-x86_64-apple-darwin -i qr-1.png -i qr-2.png
./google_authenticator_extractor-x86_64-apple-darwin -i /path/to/screenshots/
```

The batches are merged into a single list, duplicate accounts are removed, and a
warning is printed if a QR code of the export is missing.

> Note: OSX may prevent it from running with the message `“google_authenticator_extractor-x86_64-apple-darwin” cannot be opened because the developer cannot be verified.`. You need to go to `System Preferences` - `Security & Privacy` - `General` to allow it.

The output looks like as below:

```json
[{"name":"******","secret":"**********","issuer":"******"},{"name":"*****","secret":"************","issuer":""}]
```

## How to build from source

> Rust is the programming language of the project, Install Rust by following the instructions in the [official doument](https://www.rust-lang.org/tools/install).

```bash
cargo bulid
```