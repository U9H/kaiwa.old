FROM alpine:latest

WORKDIR /app
ADD . /app
EXPOSE 80

RUN apk update && \
    apk upgrade && \
    apk add rust cargo
RUN cargo run --production
