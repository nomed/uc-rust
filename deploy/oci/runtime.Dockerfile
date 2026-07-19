# syntax=docker/dockerfile:1.7

ARG RUST_VERSION=1.85.0
ARG DEBIAN_VERSION=bookworm-slim

FROM rust:${RUST_VERSION}-bookworm AS builder
WORKDIR /src

RUN apt-get update \
    && apt-get install --yes --no-install-recommends protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY apps ./apps
COPY crates ./crates
COPY proto ./proto

RUN cargo build --locked --release -p uc-cli

FROM debian:${DEBIAN_VERSION} AS runtime

ARG VERSION
ARG REVISION=unknown
ARG SOURCE=https://github.com/nomed/uc-rust

RUN test -n "${VERSION}"

LABEL org.opencontainers.image.title="UC Rust Runtime" \
      org.opencontainers.image.description="Canonical Rust/Tonic runtime for UC Operations" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.revision="${REVISION}" \
      org.opencontainers.image.source="${SOURCE}" \
      org.opencontainers.image.licenses="Apache-2.0"

RUN groupadd --gid 10001 uc \
    && useradd --uid 10001 --gid 10001 --no-create-home --home-dir /nonexistent --shell /usr/sbin/nologin uc

COPY --from=builder --chown=10001:10001 /src/target/release/uc-cli /usr/local/bin/uc-runtime

USER 10001:10001
ENV UC_GRPC_ADDR=0.0.0.0:50051 \
    UC_LOG_LEVEL=info
EXPOSE 50051

ENTRYPOINT ["/usr/local/bin/uc-runtime"]
CMD ["serve-grpc"]
