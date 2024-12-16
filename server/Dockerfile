FROM shared-rust AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release || true
RUN rm -rf src

COPY ./ ./
RUN echo "// modifying a file to invalidate cargo cache" >> src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM alpine:3.20

WORKDIR /app

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/server ./

EXPOSE 8080

CMD ["/app/server"]
