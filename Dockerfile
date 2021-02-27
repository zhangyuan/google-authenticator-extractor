# syntax=docker/dockerfile:experimental

FROM rust:1.49.0-slim-buster

WORKDIR /app

COPY Cargo.toml .

RUN echo "fn main() {}" > dummy.rs && \
  sed -i 's#src/main.rs#dummy.rs#' Cargo.toml && \
  cargo build --release && \
  sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .

RUN ./build.sh