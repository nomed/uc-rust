# syntax=docker/dockerfile:1.7

ARG RUST_VERSION=1.85.0
ARG DEBIAN_VERSION=bookworm-slim

FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-bookworm AS builder
ARG TARGETARCH
WORKDIR /src

RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
        protobuf-compiler \
        gcc-aarch64-linux-gnu \
        libc6-dev-arm64-cross \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY apps ./apps
COPY crates ./crates
COPY proto ./proto

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/src/target \
    set -eux; \
    case "${TARGETARCH}" in \
      amd64) target='x86_64-unknown-linux-gnu'; linker='gcc' ;; \
      arm64) target='aarch64-unknown-linux-gnu'; linker='aarch64-linux-gnu-gcc' ;; \
      *) echo "unsupported TARGETARCH: ${TARGETARCH}" >&2; exit 1 ;; \
    esac; \
    rustup target add "${target}"; \
    mkdir -p .cargo /out; \
    printf '[target.%s]\nlinker = "%s"\n' "${target}" "${linker}" > .cargo/config.toml; \
    cargo build --locked --release --target "${target}" -p uc-cli; \
    cp "target/${target}/release/uc-cli" /out/uc-runtime

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

COPY --from=builder --chown=10001:10001 /out/uc-runtime /usr/local/bin/uc-runtime

USER 10001:10001
ENV UC_GRPC_ADDR=0.0.0.0:50051 \
    UC_LOG_LEVEL=info
EXPOSE 50051

ENTRYPOINT ["/usr/local/bin/uc-runtime"]
CMD ["serve-grpc"]
