FROM rust:alpine3.20 AS base

RUN apk add --update-cache \
    libressl-dev \
    musl-dev \
  && rm -rf /var/cache/apk/*

FROM base AS prod-builder

WORKDIR /app

COPY ./Cargo.lock ./Cargo.toml ./
COPY ./src ./src
COPY ./views ./views
COPY ./migration ./migration

RUN cargo build --release

FROM base AS dev

WORKDIR /app

RUN cargo install cargo-watch

ENTRYPOINT [ "cargo", "watch", "-x", "run" ]

FROM alpine:3.20 AS prod

WORKDIR /app
COPY --from=prod-builder /app/target/release/igdbc igdbc

ENTRYPOINT [ "/app/igdbc", "run" ]
