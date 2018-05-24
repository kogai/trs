FROM rust:1.26.0

RUN apt-get update

WORKDIR /app
ADD . /app

ARG GOOGLE_CLOUD_PLATFORM_API_KEY
ARG OXFORD_API_ID
ARG OXFORD_API_KEY

# RUN cargo build --release --bin server
RUN cargo build --bin server && \
  mkdir -p /app/bin && \
  mv /app/target/debug/server /app/bin/server && \
  cargo clean && \
  ls -l /app/bin

EXPOSE 3000

# CMD ["./target/release/server"]
CMD ["/app/bin/server"]
