# ADR-0011: Failure engineering and agent-ready diagnostics

- Status: Accepted
- Date: 2026-07-18
- Governing issue: #28

## Context

UC Rust must remain diagnosable under CI, test, deployment and runtime failures. Opaque errors, unstructured logs and incomplete issue reports create repeated investigation work and prevent agents from resolving problems safely.

## Decision

UC Rust adopts failure engineering as a first-class architectural concern.

1. Production errors are typed by layer and retain causal chains.
2. Domain and application errors remain provider-neutral.
3. Adapters map internal errors to transport or provider representations without losing safe context.
4. Production control flow must not rely on panic, `unwrap` or `expect`.
5. Logs, traces and metrics are structured and correlated using operation, trace, span, causation and idempotency identifiers where applicable.
6. Secrets, credentials, payment data, personal data and sensitive payload fields are redacted by construction.
7. CI and runtime diagnostics produce a sanitized reproducibility bundle.
8. Automatic GitHub issue generation uses a deterministic fingerprint and updates an existing matching issue rather than creating duplicates.
9. An automatically generated issue is permitted only when it contains enough evidence for an agent to reproduce and verify the fix.
10. Retry, timeout, circuit-breaker, bulkhead, backpressure and degradation behavior must be explicit at the owning port or application boundary.

## Agent-ready issue contract

An automated issue must include:

- deterministic fingerprint;
- affected release, commit and environment;
- observed and expected behavior;
- exact reproduction command or executable failing test;
- minimized sanitized fixture;
- typed error chain;
- relevant structured logs and trace identifiers;
- affected use case, port or adapter;
- severity, impact and confidence;
- acceptance criteria;
- verification evidence required for closure.

## Consequences

- Error types and diagnostic metadata become part of architecture and testing.
- Failure paths require the same coverage and documentation as success paths.
- Observability must be designed with contracts rather than added as arbitrary log statements.
- Automated issue creation remains low-noise and actionable.
- Fault injection is required for critical failure and recovery behavior.

## Rejected alternatives

- Plain string errors across boundaries.
- Unstructured logging as the primary diagnostic source.
- Creating a new issue for every failed workflow run.
- Including raw production payloads or secrets in diagnostics.
- Treating logging and error handling as implementation details of individual adapters.
