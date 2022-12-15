FROM rust:slim-buster as builder

RUN apt-get update \
    && apt-get --no-install-recommends -y install libssl-dev=1.1.1n-0+deb10u3 pkg-config=0.29-6 \
    && rm -rf /var/lib/apt/lists/*

COPY shared /build/shared
COPY igdbc /build/igdbc
WORKDIR /build/igdbc

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update \
    && apt-get --no-install-recommends -y install openssl=1.1.1n-0+deb10u3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /build/igdbc/target/release/igdbc igdbc

ENTRYPOINT [ "/app/igdbc", "run" ]
