FROM jo3mccain/container:gorust as builder-base

RUN rustup toolchain install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    rustup default nightly

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release --verbose

FROM photon as latest

ENV SERVER_PORT=8999

COPY --from=builder /app/target/release/chaos /bin/chaos

EXPOSE ${SERVER_PORT}/tcp
EXPOSE ${SERVER_PORT}/udp

CMD ["chaos"]
