FROM scsys/devspace:latest as builder

RUN rustup default nightly

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release --verbose

FROM photon

ENV SERVER_PORT=8999

COPY --from=builder /app/target/release/chaos /bin/chaos

EXPOSE ${SERVER_PORT}/tcp
EXPOSE ${SERVER_PORT}/udp

CMD ["chaos"]
