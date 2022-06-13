FROM jo3mccain/crust as builder

RUN rustup default stable

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release --package chaos --bin chaos

FROM debian:buster-slim

COPY --from=builder /project/target/release/chaos /project/chaos

ENV DEV_MODE=false \
    PORT=9999

EXPOSE ${PORT}
CMD ["./project/chaos"]