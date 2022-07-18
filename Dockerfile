FROM jo3mccain/rusty as builder

ADD bin/chaos /app
WORKDIR /app

COPY bin/chaos .
RUN cargo build --release --verbose --color always

FROM debian:buster-slim as application

COPY --from=builder /app/target/release/aether /aether

ENTRYPOINT ["./aether"]