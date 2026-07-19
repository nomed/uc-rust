# OCI image contract

Issue: #82

## Deployable unit

The Runtime Foundation is packaged as two independently built OCI images that are released with the same immutable version and source revision:

- `uc-runtime`: Rust/Tonic canonical runtime;
- `uc-gateway`: generated Go REST/JSON adapter, OpenAPI endpoint, and Swagger UI.

They execute as two containers in one Kubernetes Pod. The gateway reaches the runtime only through Pod-local gRPC on `127.0.0.1:50051`. The Service exposes only the gateway HTTP port.

## Required image properties

Both images must:

- use multi-stage builds;
- contain only runtime-required files;
- run as a numeric non-root user;
- support a read-only root filesystem;
- have deterministic entrypoints and explicit exposed ports;
- carry OCI source, revision, version, title, and description labels;
- build for `linux/amd64` and `linux/arm64`;
- terminate cleanly within the Kubernetes grace period.

The gateway image must generate and embed the protobuf, gRPC-Gateway, OpenAPI, and Swagger assets during its build. Generated sources are build products rather than repository inputs.

## Release identity

A release consists of a pair of images sharing:

- semantic version;
- Git commit revision;
- build timestamp policy;
- registry namespace;
- architecture set.

The Kubernetes manifest must reference the same immutable tag or digest for the pair. Mutable environment-specific tags are outside this contract.

## Verification

CI must prove:

1. both image builds complete for the supported architectures;
2. image configuration matches the expected user, entrypoint, ports, and OCI labels;
3. the two containers start together with read-only root filesystems;
4. `/healthz`, `/readyz`, `/openapi.json`, and `/swagger/` behave correctly;
5. REST requests reach the canonical Rust Operation through generated gRPC-Gateway code;
6. canonical validation and cancellation errors preserve their HTTP mappings;
7. readiness becomes unavailable after the runtime container stops while gateway liveness remains healthy;
8. SIGTERM produces bounded clean shutdown evidence.

Registry publication, Helm packaging, ingress, TLS, autoscaling, and production policy are intentionally deferred.