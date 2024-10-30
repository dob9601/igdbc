FROM rust:alpine3.20 AS builder

RUN apk add --update-cache \
    libressl-dev \
    musl-dev \
  && rm -rf /var/cache/apk/*

COPY . /build/igdbc
WORKDIR /build/igdbc

RUN cargo build --release

FROM alpine:3.20 AS prod

WORKDIR /app
COPY --from=builder /build/igdbc/target/release/igdbc igdbc

ENTRYPOINT [ "/app/igdbc", "run" ]
