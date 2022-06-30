FROM jo3mccain/rusty as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release --workspace --quiet --color always

FROM debian:buster-slim as application

ENV CARGO_PKG_NAME = chaos \
    DEV_MODE=false \
    CLUSTER_PORT=9099

COPY --from=builder /project/target/release/$CARGO_PKG_NAME /$CARGO_PKG_NAME

EXPOSE $CLUSTER_PORT/tcp
EXPOSE $CLUSTER_PORT/udp

ENTRYPOINT ["./$CARGO_PKG_NAME"]