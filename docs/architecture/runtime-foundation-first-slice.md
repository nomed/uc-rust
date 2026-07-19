# Runtime Foundation first vertical slice

Issue: #70  
Governing epic: #46  
Gate: #54  
Architecture contract: `.context/rfcs/RFC-0002-runtime-foundation.md`

## Dependency direction

```text
uc-operation  <-  uc-runtime  <-  uc-adapters  <-  uc-cli
                      ^                |
                      |                +-- generated protobuf boundary
                      |
                  canonical Ping

uc-config  -------------------------------> uc-cli composition root
```

`uc-operation` contains transport-neutral canonical contracts and execution context. `uc-runtime` owns the application Operation implementation. `uc-adapters` maps protobuf and REST payloads to and from canonical models. `uc-cli` is the composition root and independent adapter proof.

## Configuration hierarchy

The effective configuration is assembled once by `uc-config`:

```text
defaults < config file < environment < CLI arguments
```

Supported environment overrides are `UC_GRPC_ADDR`, `UC_GATEWAY_ADDR`, and `UC_LOG_LEVEL`. The `uc config` command emits effective values and provenance. Secret fields are intentionally absent from this first slice; future secret fields must be required or secret references and must be redacted.

## Running

```bash
cargo run -p uc-cli -- --config config/runtime.toml config
cargo run -p uc-cli -- ping hello --tenant demo --correlation-id example-1
cargo run -p uc-cli -- --config config/runtime.toml serve-grpc
cargo run -p uc-cli -- --config config/runtime.toml serve-gateway
```

Gateway example:

```bash
curl -X POST http://127.0.0.1:8080/v1/ping \
  -H 'content-type: application/json' \
  -d '{"message":"hello","tenant_id":"demo","identity":"curl","correlation_id":"example-2"}'
```

The gateway is a sidecar-style HTTP/JSON adapter that invokes the generated gRPC client; it does not implement business behavior. The CLI invokes the same canonical Operation independently.

## Evidence

The dedicated CI workflow checks formatting, workspace compilation, tests, protobuf lint/breaking changes, and direct environment access outside `uc-config`. Shared semantic behavior is exercised by the canonical Operation tests and adapter compilation; richer transport conformance fixtures remain subsequent M1 work under #46.
