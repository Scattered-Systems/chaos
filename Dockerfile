FROM jo3mccain/rusty as builder

RUN rustup default stable

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release -p chaos

FROM debian:buster-slim as application

COPY --from=builder /project/target/release/chaos-cli /chaos-cli

ENV DEV_MODE=false \
    PORT=8888

EXPOSE ${PORT}
CMD ["./chaos-cli"]