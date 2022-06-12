FROM ubuntu as base

RUN apt-get update -y
RUN apt-get install -y build-essential curl nodejs npm yarn
RUN apt-get update -y

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

FROM base as builder-base
RUN apt-get install -y cmake pkg-config
RUN apt-get install -y libatk1.0.0 libcairo2-dev libffi-dev libglib2.0-dev libpcre2-dev libsdl-pango-dev
RUN rustup toolchain install nightly && rustup target add wasm32-unknown-unknown --toolchain nightly
RUN rustup toolchain install stable-gnu
RUN rustup default stable
RUN apt-get update -y

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release --package chaos --bin chaos

FROM debian:buster-slim

COPY --from=builder /app/target/release/chaos /chaos

ENV DEV_MODE=false \
    PORT=9999

EXPOSE ${PORT}
CMD ["./chaos"]