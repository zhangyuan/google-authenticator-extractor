# syntax=docker/dockerfile:experimental

FROM rust:1.49.0-slim-buster

WORKDIR /app

RUN apt-get update

RUN apt install -y protobuf-compiler && protoc --version

COPY Cargo.toml Cargo.lock /app/

RUN echo "fn main() {}" > dummy.rs && \
  sed -i 's#src/main.rs#dummy.rs#' Cargo.toml && \
  cargo build --release && \
  sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .

RUN ./build.sh