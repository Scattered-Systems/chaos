FROM ubuntu as builder-base

RUN apt-get update

RUN apt-get install -y \
    build-essential \
    cmake \
    curl

RUN apt-get update

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

FROM builder-base as builder

ADD . /project
WORKDIR /project

COPY . .
RUN cargo build --package chaos --bin chaos

FROM debian:buster-slim

COPY --from=builder /project/target/release/chaos /chaos

ENV DEV_MODE=false \
    PORT=9999

EXPOSE ${PORT}
ENTRYPOINT ["./chaos"]