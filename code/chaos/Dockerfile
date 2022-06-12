FROM ubuntu as builder-base

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl

# Update new packages
RUN apt-get update
RUN apt-get install -y cmake

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

FROM builder-base as builder

RUN update-ca-certificates

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/chaos /chaos

ENV DEV_MODE=false \
    PORT=8000
EXPOSE ${PORT}
CMD ["./chaos"]