FROM rust as builder

RUN update-ca-certificates

ADD . /appseed
WORKDIR /appseed

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /appseed/target/release/chaos /appseed/chaos

ENV PORT=8000
EXPOSE ${PORT}
CMD ["./appseed/chaos"]