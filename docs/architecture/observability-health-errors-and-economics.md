# Observability, Health, Errors and Economic Evidence

- Status: Design baseline
- Governing decision: ADR-0026
- Governing issue: #49

## Purpose

Define provider-neutral contracts that make every Operation outcome diagnosable, health-aware and economically attributable across central, edge, native, delegated and composed execution.

## Evidence Envelope

Every emitted observation uses a common envelope with stable schema and explicit classification. Required conceptual fields are:

- evidence ID, type, schema version and timestamp;
- invocation, correlation and optional causation IDs;
- Operation identity/version and canonical disposition;
- runtime instance/profile/location and configuration revision;
- optional capability realization/provider/binding/stage identity;
- technical impact and retail-operational impact;
- privacy class, retention class and redaction state;
- economic attribution reference and completeness.

The machine-readable baseline is `governance/schemas/evidence-envelope.schema.json`.

## Signal contracts

### Structured events

Events use stable names and schemas. They describe lifecycle transitions, binding decisions, Operation outcomes, degraded modes, reconciliation and security/economic evidence. Free text is supplemental and bounded.

### Traces

The canonical span hierarchy is:

```text
runtime admission
  -> Operation invocation
    -> authorization / transaction policy
    -> capability binding
      -> realization or pipeline stage(s)
    -> effect publication / reconciliation
  -> canonical disposition
```

Trace context is propagated across network/process boundaries using an adapter-neutral carrier. Provider trace IDs may be linked but do not replace UC correlation identities.

### Metrics

Metrics use low-cardinality dimensions such as Operation ID, disposition, runtime profile, realization kind, provider class, error class and health state. Tenant, invocation, basket, customer, order, device and store identifiers are prohibited as metric labels unless explicitly aggregated into a bounded governed dimension.

### Logs

Logs are structured evidence views. They must not be the only source for canonical outcome, health or economic attribution. Payload logging is deny-by-default.

## Canonical failure bundle

A failure bundle contains:

1. canonical error class/code and public-safe message;
2. invocation/correlation references;
3. failing component, dependency or realization;
4. retryability, idempotency and reconciliation disposition;
5. deadline/cancellation and partial/indeterminate state;
6. configuration, manifest, contract and binding revisions;
7. bounded protected diagnostics and stack/provider references;
8. technical and retail-operational impact;
9. accountable owner, runbook and escalation class;
10. privacy/redaction and retention policy.

The application receives only the canonical error/disposition. Protected diagnostics are available to authorized operational tooling.

## Health contributor contract

Each contributor reports:

- contributor ID/type and observation time;
- state: `healthy`, `degraded`, `unavailable`, `unknown`;
- freshness/expiry;
- affected Operations and profiles;
- reason code and evidence reference;
- authority, offline, legal/fiscal and security implications;
- whether it blocks readiness or only narrows capability operability.

### Aggregation rules

1. Liveness is never inferred from dependency availability.
2. Runtime readiness is evaluated against the profile-mandatory Operation set.
3. Capability operability is evaluated per Operation and mode.
4. Retail-operational health is journey-oriented and may remain acceptable while an optional technical dependency is unavailable.
5. `unknown` cannot be silently treated as healthy after freshness expiry.
6. A legal, fiscal, authority or security blocker cannot be downgraded by an aggregate average.

## Retail-operational health

The runtime can expose journey health such as:

- basket mutation operational;
- commercial calculation native/delegated/degraded;
- checkout admissible;
- payment/fiscal authority available;
- durable order/effect publication operational;
- store offline journey supported under current revisions.

This is not BI. It is an operational statement about whether supported retail outcomes can be safely completed.

## Economic correlation

An Economic Observation links technical consumption to an accepted economic unit. Minimum fields:

- Operation and disposition;
- realization/provider/stage;
- runtime profile/location class;
- duration and invocation count;
- CPU time, allocations/memory class, storage and network usage where measurable;
- provider-billed unit or estimated usage source;
- economic unit ID, allocation rule and model revision;
- completeness/confidence and missing dimensions.

Costs may be aggregated later by basket mutation, commercial calculation, checkout, order, store/day, tenant/month or other governed units. Raw cost numbers are not embedded in business contracts.

## Privacy and security

Classification levels:

- `public_operational`;
- `internal_operational`;
- `confidential`;
- `restricted`.

Secrets, authentication tokens, payment data and full customer/business payloads are prohibited. Identifiers are minimized, hashed/tokenized where correlation is needed, and access-controlled. Diagnostic access is auditable.

## Cardinality and telemetry budgets

Every signal family declares:

- permitted dimensions and maximum cardinality;
- event/trace sampling policy;
- retention and export destinations;
- maximum CPU, memory, queue, network and storage overhead;
- behavior under exporter outage/backpressure;
- evidence classes that may never be sampled away.

Exporter outage must not create unbounded memory. Critical evidence uses bounded durable buffering only where explicitly required.

## Vendor neutrality

OpenTelemetry-compatible exporters are a likely realization, not the semantic contract. Application/domain crates depend only on typed observation ports and canonical evidence types. Exporters, collectors and vendor agents remain replaceable adapters.

## Failure/degradation matrix

| Situation | Canonical outcome | Health impact | Required evidence |
|---|---|---|---|
| telemetry exporter unavailable | business outcome unchanged | observability degraded | exporter failure, bounded buffer/drop counts |
| delegated provider timeout before acknowledgement | timed out or dependency unavailable | capability degraded/unavailable | deadline, retry policy, provider and binding revision |
| delegated outcome indeterminate | indeterminate | capability degraded; reconciliation required | idempotency, provider correlation and reconciliation owner |
| optional metrics exporter overloaded | outcome unchanged | no readiness impact | sampling/drop evidence and budget breach |
| audit/fiscal evidence sink unavailable where mandatory | reject or block safely | affected Operation unavailable | authority/legal blocker and runbook |
| WAN unavailable at store edge | profile-specific | central-dependent capabilities unavailable; local subset may remain operational | offline mode, active revisions and journey health |

## Acceptance evidence

Detailed executable cases are in `docs/testing/observability-failure-and-economic-test-plan.md`.