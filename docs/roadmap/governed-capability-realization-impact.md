# Governed Capability Realization — UC Rust 1.0 roadmap impact

- Status: Proposed baseline
- Governing issue: #68
- Governing decision: ADR-0024
- Parent roadmap: `docs/roadmap/uc-rust-1.0-scope-and-traceability.md`

## Scope amendment

Governed Capability Realization is a platform characteristic, not a feature limited to pricing, loyalty, customer enquiry or invoicing.

Any canonical capability Operation may be realized as:

- native;
- delegated;
- composed;
- pipeline;
- hybrid.

UC Rust retains semantic authority. Execution and operational authority may belong to another system when explicitly declared.

## 1.0 commitment

M1 must establish:

1. canonical Capability, Operation and Realization identities;
2. the Capability Binder contract;
3. a versioned realization manifest;
4. native and delegated realizations passing the same semantic fixtures;
5. explicit deadline, retry, idempotency and indeterminate-outcome rules;
6. offline and fallback decision rules;
7. provider/realization observability and economic attribution;
8. architecture tests preventing provider leakage and adapter bypass.

M3 must prove commercial calculation through both a native realization and an RGK-compatible delegated realization without changing the canonical Operation contract.

M4 must prove an external authority boundary, using invoice/fiscal issuance or an equivalent capability where the external realization owns the authoritative outcome.

M5–M6 must include realization compatibility, rollout, rollback, drift and provider migration evidence.

## Critical-path amendment

```text
#53 planning baseline
  -> #47 Operation contract
  -> #68 Governed Capability Realization / ADR-0024
  -> #48 lifecycle and composition
  -> #49 observability and economics
  -> #51 delivery adapters
  -> #52 extension registration
  -> #54 runtime gate
```

Issue #68 is therefore a P0 dependency of the runtime foundation gate. Extension registration (#52) remains distinct: it governs packaging and registration, while #68 governs how a canonical Operation is realized.

## Capability-matrix amendment

Add the following platform responsibility to the 1.0 matrix:

| Capability / responsibility | Record | Release | Profiles | Authority | Proof | Scope |
|---|---|---|---|---|---|---|
| Governed Capability Realization | ADR-0024 + planned RRR | M1 | central, store-edge | semantic authority remains canonical; execution/operational authority declared per realization | same Operation and fixtures across native and delegated realizations | in-scope platform characteristic |

The existing Pricing and promotion calculation row must be interpreted as realization-independent. RGK is an initial delegated proving adapter, not the definition of pricing and not a mandatory dependency.

## Deferred versus delegable

A native implementation may remain deferred while the canonical capability is available through an accepted external realization. This applies to current and future capabilities, including but not limited to loyalty, customer, invoicing, tax, payment, inventory, fulfillment, search, notification, fraud and specialized AI.

Delegation does not bypass roadmap governance. Each realization still requires accepted semantics, authority, compatibility, security, offline behavior, quality budgets and economic evidence.

## Acceptance impact

The parent roadmap and #53 cannot be accepted until:

- ADR-0024 is accepted;
- #12 includes the Capability Binder and realization boundaries;
- runtime RFCs include realization lifecycle and invocation behavior;
- #68 has a complete design and evidence plan.
