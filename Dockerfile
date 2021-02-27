# syntax=docker/dockerfile:experimental

FROM rust:1.49.0-slim-buster

WORKDIR /app

RUN apt-get update

RUN apt-get install -y protobuf-compiler build-essential mingw-w64

RUN rustup toolchain install stable-x86_64-unknown-linux-gnu && \
 rustup toolchain install stable-x86_64-pc-windows-gnu

COPY Cargo.toml Cargo.lock /app/

RUN echo "fn main() {}" > dummy.rs && \
  sed -i 's#src/main.rs#dummy.rs#' Cargo.toml && \
  cargo build --release && \
  sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .

RUN ./build.sh && ./build.sh x86_64-pc-windows-gnu x86_64-unknown-linux-gnu