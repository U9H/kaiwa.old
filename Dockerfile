FROM alpine:latest

RUN apk update && \
    apk upgrade && \
    apk add rust cargo \
    apk add postgresql-dev \
    rustup default nightly
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel setup

EXPOSE 80
WORKDIR /app

RUN cargo run --release --verbose
