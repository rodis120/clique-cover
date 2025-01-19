FROM shared-rust AS builder

WORKDIR /build/wrapper

COPY wrapper/Cargo.toml wrapper/Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release || true
RUN rm -rf src

COPY wrapper .
RUN echo "// modifying a file to invalidate cargo cache" >> src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release

WORKDIR /build/solution-test
COPY solution-test .
RUN make

FROM alpine:3.20

RUN apk add --no-cache perf

WORKDIR /app

COPY --from=builder /build/wrapper/target/x86_64-unknown-linux-musl/release/algorithm-base ./
COPY --from=builder /build/solution-test/test ./solution-test
