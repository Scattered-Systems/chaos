FROM jo3mccain/rusty:nightly as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release -q

FROM photon as rpc

ENV RPC_PORT=8000

COPY --from=builder /app/target/release/chaos-rpc /chaos-rpc

EXPOSE ${RPC_PORT}/tcp
EXPOSE ${RPC_PORT}/udp

ENTRYPOINT ["./chaos-rpc"]

FROM photon as application

ENV PORT_MAINNET=9001

COPY --from=builder /app/target/release/chaos /chaos

EXPOSE ${PORT_MAINNET}/tcp
EXPOSE ${PORT_MAINNET}/udp

ENTRYPOINT ["./chaos"]