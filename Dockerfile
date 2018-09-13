FROM alpine:latest

RUN apk update && \
    apk upgrade && \
    apk add rust cargo

EXPOSE 80
# WORKDIR /app




# RUN cargo run --release --verbose
