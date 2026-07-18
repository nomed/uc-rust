# Capability Realization Semantics and Evidence

- Status: Reviewable baseline
- Governing decision: ADR-0024
- Governing issue: #68

## Invocation and failure matrix

| Situation | Canonical disposition | Retry rule | Required evidence |
|---|---|---|---|
| Rejected before provider send | deterministic failure | allowed after corrected input/policy | binding decision and rejection reason |
| Transport failure known before acceptance | provider unavailable | retry only within declared budget | attempt count, deadline and circuit state |
| Provider accepted invocation and connection is lost | indeterminate unless provider proves otherwise | reconcile before retry unless idempotency contract proves safety | invocation key, provider receipt or reconciliation result |
| Provider returns canonical business rejection | business rejection | no transport retry | mapped canonical error plus protected provider diagnostic |
| Provider returns unsupported context | unsupported realization/context | rebind only if policy allows another semantically valid realization | manifest capability mismatch evidence |
| Deadline exceeded with cancellation confirmed | deadline exceeded | retry according to idempotency policy | cancellation acknowledgement |
| Deadline exceeded without cancellation confirmation | indeterminate | reconcile/manual resolution | authority-specific reconciliation evidence |
| Partial pipeline completion | explicit partial/failed state | stage policy controls compensate/reconcile | completed stages, authority contributions and compensation state |
| Duplicate invocation | original canonical result or duplicate disposition | never duplicate business effect | idempotency record and retained result reference |

## Retry and idempotency rules

1. A retry policy is part of the realization contract, not generic middleware behavior.
2. Every mutation-capable Operation declares an idempotency scope and retention period.
3. `reconcile-before-retry` is mandatory when an external authority may have committed an outcome but acknowledgement is missing.
4. Retry budgets are bounded by the original Operation deadline.
5. A provider-specific duplicate response is translated to the canonical original-result or duplicate disposition.
6. A pipeline cannot replay a completed non-idempotent stage without compensation or reconciliation.

## Offline and fallback matrix

| Declared mode | Permitted behavior | Required constraints |
|---|---|---|
| `unavailable` | fail safely before execution | stable error, no hidden degraded behavior |
| `local-realization` | bind to approved local/native realization | semantic revision, authority and conformance equivalence |
| `stale-snapshot` | execute using governed local data | explicit freshness bound and outcome marking where required |
| `queue-defer` | durably record intent for later execution | Operation semantics permit delay; idempotency and expiry defined |
| `degraded-result` | return a weaker but canonical explicit outcome | degradation represented in canonical result and accepted by policy |
| `manual-resolution` | create auditable operational work | owner, SLA, evidence and reconciliation path |

Fallback is forbidden when it would silently alter legal issuance, fiscal authority, payment authorization, security, tenant isolation or finality semantics.

## Authority decision table

| Example | Source-data authority | Calculation/decision authority | Final/system-of-record authority |
|---|---|---|---|
| Native commercial calculation | declared catalog/customer/promotion authorities | native pricing realization | basket/checkout authority according to capability contract |
| RGK commercial calculation | upstream product/customer/promotion sources as declared | RGK adapter realization for calculated outcome | UC Rust basket remains authoritative for basket state |
| External customer enquiry | external customer platform | external customer platform | external customer platform unless contract marks result as snapshot/advisory |
| External invoice issuance | UC Rust checkout/order plus fiscal inputs | external issuer | external issuer owns legal document identity and issuance evidence |
| Pipelined pricing | each source/stage declares contribution | governed graph | finalizer identified by capability contract; no incidental last-writer authority |

## Composition and pipeline proof

The first governed pipeline proof is:

```text
CalculateCommercialOutcome
  -> validate-canonical-context (native validator)
  -> calculate-commercial-adjustments (delegated RGK-compatible adapter)
  -> validate-rounding-and-fiscal-constraints (native validator)
  -> finalize-canonical-pricing-outcome (native finalizer)
```

Rules:

- the delegated stage calculates commercial adjustments but does not own basket persistence;
- the first native stage rejects invalid or unauthorized input before provider transmission;
- the final native stage cannot silently rewrite provider adjustments; it may accept, reject or return an explicit canonical conflict;
- provider timeout after acceptance is indeterminate and requires reconciliation;
- every stage has an independent deadline inside the Operation budget;
- trace and economic correlation span the whole graph and retain stage attribution.

## Semantic conformance kit

Each canonical Operation provides a provider-neutral test kit containing:

### Contract fixtures

- canonical valid inputs and expected outcomes;
- boundary values, rounding and version cases;
- canonical business rejections;
- unsupported-context cases;
- authority/finality expectations;
- provider-neutral explanation and evidence assertions.

### Behavioral scenarios

- repeated invocation with the same idempotency key;
- concurrent duplicate invocation;
- timeout before provider acceptance;
- timeout after possible provider acceptance;
- cancellation supported and unsupported;
- provider unavailability and circuit opening;
- compatibility mismatch;
- tenant and jurisdiction isolation;
- offline/fallback behavior;
- rollout and rollback across compatible revisions.

### Architecture assertions

- delivery adapters depend only on canonical Operation contracts;
- application/domain modules contain no provider SDK types;
- provider adapters cannot bypass authorization, observability, audit or economics ports;
- manifests are schema-valid and approved before binding;
- composition stages form an acyclic declared graph;
- no provider-specific error escapes the canonical boundary.

## Native/delegated equivalence proof

M1/M3 evidence must run the same canonical fixtures against:

1. a native realization;
2. an in-memory delegated test provider using a real adapter boundary;
3. optionally an RGK-compatible adapter as the concrete commercial-calculation case.

Equivalence means equal canonical meaning, not byte-identical provider diagnostics. The proof compares:

- canonical outcome state;
- monetary values, precision and rounding where applicable;
- applied adjustment identities and explanation references;
- stable canonical errors;
- authority and finality markers;
- idempotency behavior;
- required trace/economic evidence.

A provider that supports only a narrower context may still conform if the limitation is declared in its manifest and rejected during binding rather than discovered after semantic execution.

## Compatibility, rollout and migration

- Binding requires compatible capability semantic revision, provider contract, adapter revision and runtime envelope.
- New provider revisions enter through canary/tenant/store policy and retain the previous approved binding for rollback.
- Rollback is forbidden after an irreversible external authority transition unless the capability defines migration/reconciliation semantics.
- Long-running Operations retain the original binding revision until completion or governed migration.
- Provider replacement requires conformance evidence and an authority/data migration plan; configuration alone is insufficient when external state or legal identities are involved.

## Acceptance evidence for issue #68

The design is complete when the repository contains:

- ADR-0024;
- architecture specification;
- machine-validatable realization manifest schema;
- validator command;
- Capability Binder contract;
- routing, authority, failure, idempotency and offline rules;
- native/delegated equivalence test design;
- composed/pipeline proof design;
- explicit impacts on Operation First, Runtime Foundation, Pricing and 1.0 planning.

Executable implementation evidence remains an M1/M3 gate obligation and is not falsely claimed by this architecture issue.
