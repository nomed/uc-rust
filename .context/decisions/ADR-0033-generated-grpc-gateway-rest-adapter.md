# ADR-0033 — Generated gRPC-Gateway REST adapter

- Status: Accepted
- Date: 2026-07-19
- Governing issue: #80
- Related: ADR-0028, ADR-0030, ADR-0032, RFC-0002

## Context

The Runtime Foundation exposes canonical application behavior through transport-neutral Operations and a Tonic gRPC adapter. A REST/JSON delivery path is also required, but implementing per-method routing, JSON transcoding, error conversion, metadata propagation, OpenAPI generation, and protobuf evolution by hand would duplicate responsibilities already solved by the standard gRPC ecosystem.

A previous Axum gateway was useful as a conformance spike, but retaining it as the default adapter would create a second handwritten public contract and an avoidable maintenance burden.

## Decision

Adopt **gRPC-Gateway v2** as the generated REST/JSON adapter for Rust/Tonic services.

- Protobuf is the single source of truth for gRPC, REST bindings, and OpenAPI.
- HTTP bindings are declared through `google.api.http` annotations or an explicit external service configuration when annotations cannot be embedded.
- Buf owns reproducible generation of Go protobuf, gRPC-Gateway, and OpenAPI artifacts.
- The generated gateway delegates to the canonical gRPC service and contains no business rules.
- Canonical execution, validation, authorization, deadlines, cancellation, tracing, and error semantics remain inside the Rust runtime and transport-neutral Operations.
- Handwritten per-method REST transcoding is prohibited for normal service delivery.

## Deployment model

The gateway and Rust runtime form one logical deployable unit:

- one Kubernetes Pod or equivalent local composition unit;
- Rust/Tonic bound to loopback or a Pod-internal port;
- generated Go gateway exposed through the existing Kubernetes Service;
- one release version, readiness contract, shutdown policy, and operational ownership boundary.

The two executables remain separate processes. Embedding the Go runtime into Rust through FFI is rejected because it would increase build, runtime, observability, and failure-mode complexity without reducing the logical deployment surface.

## Required behavior

The generated adapter must preserve the canonical runtime contract:

- propagate `traceparent`, `tracestate`, correlation identity, and supported deadline metadata;
- map canonical gRPC statuses deterministically to safe HTTP problem responses;
- expose no internal error details;
- generate an OpenAPI description from the protobuf contract;
- pass live semantic conformance tests for success, invalid input, deadline expiry, tracing, and correlation;
- provide health and readiness evidence for both processes;
- support coordinated graceful shutdown.

## Consequences

- REST evolution follows protobuf evolution rather than handwritten route maintenance.
- Generated Go code and gateway infrastructure become build inputs, but not domain or application dependencies.
- The repository gains a small Go build surface dedicated exclusively to transport adaptation.
- Local development and deployment must start both the Rust service and generated gateway.
- Buf module dependencies and plugin versions must be pinned or locked reproducibly.
- CI must verify protobuf linting, compatibility, deterministic generation, OpenAPI output, gateway compilation, and live cross-transport conformance.
- Envoy gRPC-JSON transcoding remains a valid future deployment alternative where Envoy is already a platform dependency, provided it preserves the same protobuf and semantic contract.

## Rejected alternatives

### Handwritten Axum gateway

Rejected as the default because it duplicates transcoding, routing, error mapping, metadata propagation, and OpenAPI responsibilities.

### Embedded Go gateway through FFI

Rejected because it creates a fragile mixed-runtime process and provides no meaningful architectural advantage over two coordinated processes in one Pod.

### Independently deployed gateway service

Rejected for the initial runtime because it introduces an unnecessary separately discoverable service, release lifecycle, and operational boundary.
