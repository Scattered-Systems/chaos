FROM debian as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    build-essential \
    curl \
    pkg-config

FROM builder-base as builder

RUN curl -fsSL https://get.deta.dev/space-cli.sh | sh

FROM builder

ENV DETA_SPACE_TOKEN=""

RUN space login ${DETA_SPACE_TOKEN}

ENTRYPOINT  ["space" ]
