FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --release -v --workspace

FROM debian:buster-slim as runner

ENV RUST_LOG="info" \
    SERVER_PORT=8080

RUN apt-get update -y && apt-get upgrade -y 

COPY --chown=55 .config /config
VOLUME ["/config"]

COPY --from=builder /workspace/target/release/chaos /bin/chaos

FROM runner

EXPOSE 80
EXPOSE 6379
EXPOSE ${SERVER_PORT}

ENTRYPOINT [ "chaos" ]
CMD [ "system", "--up" ]
