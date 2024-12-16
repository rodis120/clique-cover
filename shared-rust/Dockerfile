FROM rust:1.82-slim-bookworm

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl musl-dev musl-tools pkg-config build-essential libssl-dev

WORKDIR /shared-rust

COPY ./ ./
