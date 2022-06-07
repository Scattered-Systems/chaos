FROM rust as builder

RUN update-ca-certificates

ADD . /appseed
WORKDIR /appseed

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /appseed/target/release/chaos_rpc /appseed/chaos_rpc

ENV PORT=8001
EXPOSE ${PORT}
CMD ["./appseed/chaos_rpc"]