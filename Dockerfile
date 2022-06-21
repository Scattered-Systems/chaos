FROM jo3mccain/rusty as builder

RUN rustup default stable

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release -p disarray --bin chaos

FROM debian:buster-slim as application

COPY --from=builder /project/target/release/chaos /chaos

ENV DEV_MODE=false \
    PORT=8888

EXPOSE ${PORT}
CMD ["./chaos"]