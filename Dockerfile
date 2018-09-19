FROM rustlang/rust:nightly

RUN apt update && \
    apt upgrade -y

WORKDIR /app
RUN apt install -y libpq-dev postgresql
RUN cargo install diesel_cli --no-default-features --features postgres
RUN service postgresql start
# RUN diesel setup
EXPOSE 3000
