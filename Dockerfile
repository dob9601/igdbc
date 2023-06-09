FROM rust:slim-buster as builder

RUN apt-get update \
    && apt-get --no-install-recommends -y install libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY shared /build/shared
COPY igdbc /build/igdbc
WORKDIR /build/igdbc

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update \
    && apt-get --no-install-recommends -y install openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Rocket.toml .
COPY --from=builder /build/igdbc/target/release/igdbc igdbc

ENTRYPOINT [ "/app/igdbc", "run" ]
