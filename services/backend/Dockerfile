FROM rust:1.79.0-alpine as builder

WORKDIR /app
COPY Cargo.toml .
COPY Cargo.lock .
COPY /src/* ./src/

RUN apk add --no-cache musl-dev
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /usr/local/bin/backend

EXPOSE 8080
CMD ["backend"]
