FROM algonet-client

COPY . .

RUN apk add --no-cache python3

ENV ALGO_NAME=divset
ENV ALGO_RUN_CMD="python3 /app/divset.py"

CMD ["/app/algorithm-base"]

