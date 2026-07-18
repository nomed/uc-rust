# ADR-0026 — Observability, Health, Errors and Economic Correlation

- Status: Accepted
- Date: 2026-07-19
- Governing issue: #49
- Parent epic: #46
- Related: ADR-0021, ADR-0024, ADR-0025, RFC-0002, #29, #43

## Context

UC Rust must diagnose every Operation outcome, expose technical and retail-operational health, and attribute runtime cost without coupling application semantics to a telemetry vendor. Central, edge and delegated realizations introduce partial failure, degraded operation, privacy constraints, high-cardinality identifiers and provider-attributed costs.

## Decision

UC Rust adopts provider-neutral observability contracts centered on a canonical **Evidence Envelope**.

Every invocation records a governed chain:

```text
Invocation
  -> Operation
    -> Capability Realization / stage
      -> component and dependency evidence
        -> canonical outcome or failure
          -> technical and retail-operational impact
            -> economic attribution
```

Application and domain code emit semantic observations through typed ports. Exporters translate those observations to OpenTelemetry, logs, metrics or other backends. Vendor SDK types do not cross the observability boundary.

## Correlation model

The stable correlation set includes:

- invocation ID;
- correlation ID;
- causation ID when applicable;
- Operation ID and semantic version;
- tenant and legal-entity pseudonymous scope;
- runtime instance, profile, location and configuration revision;
- selected realization, provider and binding revision;
- effect, job or reconciliation identity where applicable;
- economic unit and allocation dimensions.

Sensitive payloads, customer identifiers, payment data, secrets and unbounded free text are excluded by default.

## Error and failure model

Canonical errors remain those defined by ADR-0021. A **Failure Bundle** supplements the canonical error with protected operational evidence:

- stable failure class and code;
- retry and reconciliation disposition;
- failing component or realization;
- dependency state and deadline/cancellation facts;
- configuration, contract and binding revisions;
- trace references and bounded diagnostics;
- technical and retail-operational impact;
- runbook and ownership references;
- redaction and retention classification.

Provider, transport and persistence diagnostics are retained only inside the protected bundle and never become canonical application contracts.

## Health model

Health is multidimensional:

- **process liveness** — the process can make progress;
- **runtime readiness** — the profile can admit its mandatory Operation set;
- **capability operability** — a named Operation/capability is available under a declared mode;
- **dependency health** — a component or provider is within its accepted envelope;
- **retail-operational health** — the store journey or business outcome remains usable.

Health contributors report `healthy`, `degraded`, `unavailable` or `unknown`, with reason, affected Operations, authority/offline implications and freshness. Aggregation is deterministic and never hides a narrower unavailable capability behind overall process health.

## Metrics, logs and traces

- Metrics use bounded dimensions only.
- High-cardinality identities belong in traces or protected events, not metric labels.
- Logs are structured events with stable event names and schemas.
- Traces represent invocation, realization and governed pipeline stages.
- Sampling must preserve errors, indeterminate outcomes, security events and economic reconciliation evidence.
- Telemetry backpressure may reduce detail but cannot block critical business outcomes unless required for legal or security evidence.

## Economics by Design

Every invocation can emit an economic observation that links:

- Operation and outcome;
- realization/provider and stage;
- runtime profile and location;
- duration, CPU, memory/allocation, storage, network and provider usage;
- accepted economic unit;
- allocation rule and model revision;
- confidence/completeness indicator.

Economic telemetry is observational and cannot redefine business outcomes. Missing cost evidence is visible as incomplete attribution, not silently estimated as authoritative.

## Privacy, cardinality and cost budgets

Each signal declares classification, retention, sampling, cardinality and export budget. Raw business payloads are prohibited unless an accepted evidence requirement explicitly permits a minimized and protected representation.

Observability overhead has explicit CPU, memory, network, storage and provider-cost budgets. Dynamic sampling or aggregation may operate only inside diagnostic, security, legal and economic evidence constraints.

## Consequences

- Any Operation outcome can be diagnosed across adapters, runtime profiles and realizations.
- Technical health and retail-operational health remain distinct.
- Provider replacement does not require changing canonical observability contracts.
- Economic attribution becomes reproducible and realization-aware.
- Privacy and telemetry cost are architecture constraints rather than exporter settings.

## Rejected alternatives

- vendor SDK calls from application/domain code;
- free-text errors as the diagnostic contract;
- readiness equal to liveness;
- provider availability equal to complete retail-operational health;
- high-cardinality metric labels;
- logging complete requests/responses by default;
- cost attribution inferred only from aggregate infrastructure invoices;
- best-effort telemetry that silently drops indeterminate or legally relevant evidence.

## Design evidence

- `docs/architecture/observability-health-errors-and-economics.md`;
- `governance/schemas/evidence-envelope.schema.json`;
- `docs/testing/observability-failure-and-economic-test-plan.md`;
- RFC-0002 integration.

## Implementation evidence required by M1/M8

- one successful, rejected, failed and indeterminate Operation evidence chain;
- native and delegated realization trace/economic attribution;
- deterministic health aggregation and degraded edge proof;
- failure bundle redaction and diagnostic reconstruction;
- cardinality and sampling tests;
- observability CPU, memory, network, storage and cost scorecards;
- exporter replacement proof without application changes;
- privacy and retention enforcement tests.

This ADR can be accepted before executable evidence exists; gate #54 and M8 must not claim those proofs until produced.
