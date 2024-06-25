FROM rust:1.79.0-alpine

WORKDIR /app
COPY /migrations ./migrations

RUN apk add --no-cache musl-dev
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres

CMD [ "sqlx", "migrate", "run" ]
