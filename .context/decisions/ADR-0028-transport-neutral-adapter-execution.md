# ADR-0028 — Transport-Neutral Adapter Execution Model

- Status: Reviewable for acceptance
- Date: 2026-07-19
- Governing issue: #51
- Parent epic: #46
- Related: ADR-0021, ADR-0025, ADR-0026, ADR-0027, RFC-0002

## Context

UC Rust exposes canonical Operations through REST, gRPC, CLI, workers, schedulers and future delivery mechanisms. Without one mapping model, adapters can acquire business logic, diverging validation rules, incompatible errors, duplicated transaction scripts and transport-specific semantics.

## Decision

Every delivery adapter is a boundary translator around the same canonical Operation invocation contract.

```text
transport input
  -> decode
  -> transport-shape validation
  -> verified identity and context mapping
  -> canonical Operation invocation
  -> canonical disposition
  -> transport-specific encoding
```

Adapters may translate representation and protocol concerns. They do not own business decisions, transactions, repositories, provider selection, effect publication or private application procedures.

## Validation boundary

Adapter validation is limited to transport shape and safe decoding, including required fields, syntax, size limits, encoding, media type and protocol framing.

Canonical input validation, business invariants, authorization, idempotency equivalence and state conflict handling belong to the Operation and its governed policies.

The same canonical invalid input must produce the same Operation error class regardless of adapter. A malformed HTTP body or invalid protobuf frame may fail before invocation as a transport error; a decoded but semantically invalid command reaches the canonical Operation boundary.

## Context propagation

Adapters construct an immutable InvocationContext from verified and approved sources. They propagate:

- invocation, correlation and causation identity;
- authenticated actor evidence and tenant/legal scope;
- channel, touchpoint, location and device scope where accepted;
- absolute deadline and cooperative cancellation;
- idempotency key and scope when declared;
- requested Operation version;
- policy, configuration and trace/economic correlation references.

Arbitrary headers, framework request objects, raw credentials and unverified claims do not enter the canonical context.

## Outcome mapping

Adapters map canonical dispositions without changing their meaning:

- `succeeded` — successful protocol response;
- `rejected` — stable business/policy rejection;
- `failed` — known failure without authoritative success;
- `accepted` — durable asynchronous acceptance and canonical handle;
- `indeterminate` — explicit uncertainty and reconciliation reference;
- `cancelled` — safe cooperative cancellation;
- `timed_out` — deadline outcome with certainty metadata.

Transport status codes are views of canonical meaning, not the source of business semantics. Provider, persistence and exporter diagnostics remain in protected evidence and never become public adapter contracts.

## Adapter-specific rules

### REST

REST maps method, path, headers and body to one Operation descriptor. HTTP status selection follows the canonical disposition and error class. Idempotency and correlation headers are accepted only when declared. REST resources cannot bypass Operations by calling repositories.

### gRPC

gRPC methods map protobuf messages to canonical input/output contracts. gRPC deadlines and cancellation map to the InvocationContext. Status codes and error details carry stable canonical classes and safe fields only.

### CLI

CLI commands map arguments, standard input and local identity evidence to Operations. Exit codes are stable adapter mappings. Human-readable rendering is separate from machine-readable canonical output. Interactive prompts cannot change business semantics.

### Worker and scheduler adapters

Workers map Work Envelopes or accepted events to the same Operation contract under ADR-0027. Acknowledgement, retry and quarantine are work-disposition concerns; they do not create worker-specific business outcomes.

## Compatibility and versioning

Adapter revisions and protocol schemas are versioned independently from Operation semantic versions. A non-breaking adapter revision may continue to invoke the same Operation version. A breaking canonical semantic change requires a new Operation major version and an explicit adapter mapping.

Compatibility is checked across adapter revision, canonical input/output/error contracts, Operation version, runtime profile, authorization policy and realization eligibility before protected work where feasible.

## Canonical fixtures

All adapters reuse canonical Operation fixtures. Adapter-specific fixtures cover only encoding, decoding and status mapping around those canonical fixtures. Expected business outcomes are not duplicated per transport.

## Architecture enforcement

Automated dependency rules must prevent adapters from importing business repositories, concrete providers, transaction managers, domain event publishers or private business services. Adapter packages may depend on canonical contracts and runtime invocation ports only.

## Consequences

- Transport replacement does not change business semantics.
- One Operation can be proven across REST, gRPC, CLI and worker forms.
- Public errors remain stable and vendor-neutral.
- Identity, deadline, cancellation and correlation propagation are explicit.
- Adapter code remains thin, testable and replaceable.

## Rejected alternatives

- controller- or RPC-service-owned business procedures;
- separate application services per transport;
- transport status codes as the canonical error model;
- direct adapter repository/provider access;
- untyped maps as the application contract;
- copying complete request/response payloads into telemetry;
- worker-only `run()` business methods.

## Design evidence

- `docs/architecture/transport-neutral-adapter-execution.md`;
- `governance/schemas/adapter-binding.schema.json`;
- `docs/testing/transport-adapter-conformance-test-plan.md`;
- canonical Operation fixtures governed by ADR-0021.

## Implementation evidence required by M1

- one Operation invoked unchanged through at least two adapter forms;
- canonical fixture reuse with adapter-only encoding fixtures;
- identity, deadline, cancellation and correlation propagation tests;
- disposition/error mapping tests;
- architecture tests preventing adapter bypass;
- compatibility tests for adapter and Operation versions;
- trace and economic evidence proving identical canonical meaning.

This ADR can be accepted before executable M1 evidence exists; gate #54 must not claim those proofs until produced.