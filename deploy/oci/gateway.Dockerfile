# syntax=docker/dockerfile:1.7

ARG GO_VERSION=1.24.5
ARG DEBIAN_VERSION=bookworm-slim
ARG BUF_VERSION=1.57.2

FROM golang:${GO_VERSION}-bookworm AS builder
ARG BUF_VERSION
WORKDIR /src

COPY buf.yaml buf.gen.yaml ./
COPY proto ./proto
COPY gateway ./gateway

RUN go install github.com/bufbuild/buf/cmd/buf@v${BUF_VERSION} \
    && buf generate \
    && cd gateway \
    && go mod tidy \
    && CGO_ENABLED=0 go build -trimpath -ldflags='-s -w' -o /out/uc-gateway ./cmd/uc-gateway

FROM debian:${DEBIAN_VERSION} AS runtime

ARG VERSION
ARG REVISION=unknown
ARG SOURCE=https://github.com/nomed/uc-rust

RUN test -n "${VERSION}"

LABEL org.opencontainers.image.title="UC REST Gateway" \
      org.opencontainers.image.description="Generated REST/JSON adapter for the canonical Rust/Tonic runtime" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.revision="${REVISION}" \
      org.opencontainers.image.source="${SOURCE}" \
      org.opencontainers.image.licenses="Apache-2.0"

RUN groupadd --gid 10001 uc \
    && useradd --uid 10001 --gid 10001 --no-create-home --home-dir /nonexistent --shell /usr/sbin/nologin uc

COPY --from=builder --chown=10001:10001 /out/uc-gateway /usr/local/bin/uc-gateway

USER 10001:10001
ENV UC_GATEWAY_ADDR=0.0.0.0:8080 \
    UC_GRPC_ENDPOINT=127.0.0.1:50051
EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/uc-gateway"]
