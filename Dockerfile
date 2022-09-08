FROM jo3mccain/rusty:nightly as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release -q

FROM photon as api

ENV SERVER_PORT=8080

COPY --from=builder /app/target/release/chaos-rpc /chaos-rpc

EXPOSE ${SERVER_PORT}/tcp
EXPOSE ${SERVER_PORT}/udp

CMD ["./chaos-api"]

FROM photon as cli

ENV PORT_MAINNET=9001

COPY --from=builder /app/target/release/chaos /chaos

EXPOSE ${PORT_MAINNET}/tcp
EXPOSE ${PORT_MAINNET}/udp

ENTRYPOINT ["./chaos-cli"]