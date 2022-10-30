FROM rust as builder-base


RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    clang-11 \
    curl \
    wget

RUN wget https://go.dev/dl/go1.19.linux-amd64.tar.gz

RUN apt-get tar -xzf go1.19.linux-amd64.tar.gz \
    sudo mv go /usr/local \
    export PATH=$PATH:/usr/local/go/bin

FROM builder-base as builder

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --release --workspace && \
    cargo test --all-features --color always --release --verbose

FROM debian:buster-slim

ENV MODE="production" \
    SERVER_PORT=8080 \
    RUST_LOG="debug"

COPY --from=builder /workspace/target/release/gateway /bin/gateway

EXPOSE ${SERVER_PORT}/tcp
EXPOSE ${SERVER_PORT}/udp

CMD ["sandbox"]