# Runtime Foundation semantic conformance

This slice extends the canonical Operation runtime with transport-neutral tracing,
deadline and cancellation evidence.

## Canonical context

`ExecutionContext` owns correlation, W3C-compatible trace context, an optional
monotonic deadline and an explicitly injected cooperative cancellation token.
Domain and application contracts do not depend on Axum, Tonic or an
OpenTelemetry SDK.

Admission calls `ensure_active` before business work. Cancellation and deadline
expiry are distinct canonical outcomes. Operations performing longer work must
call `ensure_active` at safe cooperative checkpoints and before committing an
external effect. A future Unit of Work/outbox implementation must bind that
final check to the atomic commit boundary.

## Adapter mappings

| Canonical outcome | gRPC | REST gateway | CLI exit |
|---|---:|---:|---:|
| invalid request | INVALID_ARGUMENT | 400 | 2 |
| unauthorized | UNAUTHENTICATED | 401 | 3 |
| forbidden | PERMISSION_DENIED | 403 | 3 |
| not found | NOT_FOUND | 404 | 4 |
| conflict | ALREADY_EXISTS | 409 | 5 |
| deadline exceeded | DEADLINE_EXCEEDED | 504 | 6 |
| cancelled | CANCELLED | 408 | 7 |
| unavailable | UNAVAILABLE | 503 | 8 |
| internal | INTERNAL | 500 | 1 |

The gateway forwards `traceparent`, `tracestate` and `x-timeout-ms` to gRPC
metadata. Diagnostic responses expose only a safe title, status and correlation
identifier.

## Semantic fixtures

The initial reusable fixture context is provided by `uc_runtime::fixture_context`.
The Runtime Foundation CI proves the canonical success, invalid input, expired
deadline and caller-cancellation outcomes in-process, plus deterministic error
mapping in the gRPC and CLI adapters. Network-level fixture execution through
live gRPC and REST processes remains the next extension of this slice before
M1 evidence is considered complete.
