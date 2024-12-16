FROM shared-rust AS builder

WORKDIR /build

COPY wrapper/Cargo.toml wrapper/Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release || true
RUN rm -rf src

COPY wrapper .
RUN echo "// modifying a file to invalidate cargo cache" >> src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM alpine:3.20

WORKDIR /app

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/algorithm-base ./
