FROM rust:1.49.0-slim-buster

RUN rustup component add clippy

WORKDIR /app

RUN apt-get update

RUN apt-get install -y protobuf-compiler

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src/ && echo "fn main() {}" > src/main.rs && \
  cargo build --release && \
  rm -rf src

COPY . .

RUN ./scripts/build.sh