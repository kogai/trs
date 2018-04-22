FROM rust:1.25.0
RUN apt-get update
WORKDIR /app
ADD . /app
CMD [ "cargo", "build", "--release" ]
