FROM rust as builder

RUN update-ca-certificates && apt-get cmake

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/chaos /chaos

ENV DEV_MODE=false \
    PORT=8000
EXPOSE ${PORT}
CMD ["./chaos"]