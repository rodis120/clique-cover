FROM debian:bookworm-slim AS builder

RUN apt-get update && apt-get install -y \
    g++ \
    make \
    libc6-dev

WORKDIR /build
COPY . .
RUN g++ -static -o grafsil graf_sil.cpp


FROM algonet-client

WORKDIR /app

COPY --from=builder /build/grafsil .

ENV ALGO_NAME=grafsil
ENV ALGO_RUN_CMD="echo"

CMD ["/app/algorithm-base"]
