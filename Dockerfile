FROM jo3mccain/crust as builder

RUN rustup default stable

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --release --package chaos --bin chaos

FROM debian:buster-slim as application

COPY --from=builder /project/target/release/chaos /project/chaos

RUN apt-get update -y

RUN apt-get install -y build-essential curl

RUN apt-get install -y libc6 libc-bin libglib2.0-dev

RUN apt-get update -y

FROM application

ENV DEV_MODE=false \
    PORT=9999

EXPOSE ${PORT}
CMD ["./project/chaos"]