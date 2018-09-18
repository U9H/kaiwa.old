FROM alpine:latest

RUN apk update && \
    apk upgrade && \
    apk add rust cargo \
    apk add sqlite-libs
RUN cargo install diesel_cli --no-default-features --features postgres

EXPOSE 80
WORKDIR /app

RUN cargo run --release --verbose
