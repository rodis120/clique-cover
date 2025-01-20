FROM algonet-client

COPY . .

RUN apk add --no-cache python3 py3-pip py3-virtualenv

RUN python3 -m venv /py-venv
ENV PATH="/py-venv/bin:$PATH"
RUN pip install --no-cache-dir numpy

ENV ALGO_NAME=rga
ENV ALGO_RUN_CMD="python3 /app/RGA.py -r"

CMD ["/app/algorithm-base"]
