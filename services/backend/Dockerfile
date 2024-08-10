#----------------------------#
# Production target          #
#                            #
# Release build running      #
# in scratch image           #
#----------------------------#

FROM rust:1.80.1-alpine AS builder

WORKDIR /app
COPY . .

RUN apk add --no-cache musl-dev
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch AS release

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /usr/local/bin/backend

EXPOSE ${APP_PORT}
CMD ["backend"]

#----------------------------#
# Development target         #
#                            #
# Cargo watch -x run         #
#----------------------------#

FROM rust:1.80.1-alpine AS dev

WORKDIR /app
COPY . .

RUN apk add --no-cache musl-dev
RUN cargo install cargo-watch

EXPOSE ${APP_PORT}
CMD ["cargo", "watch", "-x", "run"]
