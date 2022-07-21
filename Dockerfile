FROM jo3mccain/rusty as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release --verbose --color always

FROM photon as application

ENV PORT=9001

COPY --from=builder /app/target/release/chaos /chaos

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp
ENTRYPOINT ["./chaos"]