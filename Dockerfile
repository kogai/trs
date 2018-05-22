FROM rust:1.26.0

RUN apt-get update

WORKDIR /app
ADD . /app

ARG GOOGLE_CLOUD_PLATFORM_API_KEY
ARG OXFORD_API_ID
ARG OXFORD_API_KEY

RUN cargo build

CMD [ "cargo", "build", "--release" ]
