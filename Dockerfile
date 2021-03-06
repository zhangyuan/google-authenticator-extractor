FROM rust:1.49.0-slim-buster

RUN rustup component add clippy

WORKDIR /app

RUN apt-get update

RUN apt-get install -y protobuf-compiler

COPY Cargo.toml Cargo.lock /app/

RUN echo "fn main() {}" > dummy.rs && \
  sed -i 's#src/main.rs#dummy.rs#' Cargo.toml && \
  cargo build --release && \
  sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .

RUN ./scripts/build.sh