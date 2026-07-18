# Release gates through 1.0

- Governing planning issue: #53
- Scope and traceability baseline: `docs/roadmap/uc-rust-1.0-scope-and-traceability.md`

Each release gate requires accepted evidence in six dimensions:

1. **Functional** — canonical Operations, fixtures and expected outcomes.
2. **Architectural** — dependency boundaries, data authority and one-place business logic.
3. **Distributed** — consistency, offline class, recovery and convergence behavior where applicable.
4. **Security** — authentication, authorization, isolation, privacy and supply-chain evidence.
5. **Operational** — health, observability, rollback, compatibility and incident reproduction.
6. **Economic** — resource consumption, attributable cost, capacity and regression evidence.

## Gate record

A release cannot be marked complete from issue closure alone. Its governing epic or gate must record:

- decision outcome and accountable approver;
- evidence links for all applicable dimensions;
- capability rows and releases covered;
- unresolved risks and accepted residual exposure;
- approved exceptions with owner, scope and expiry;
- compatibility and migration disposition;
- UC-BoK and Economics by Design disposition.

## Outcomes

A gate has one of four outcomes:

- **Approved** — all required evidence is accepted and no blocking exception exists.
- **Approved with bounded exceptions** — implementation/release may proceed only within explicit owner, scope and expiry constraints.
- **Rework required** — evidence is incomplete or a blocking decision remains unresolved.
- **Rejected or superseded** — the proposed release or architecture is not proceeding in its current form.

Silence, elapsed time, merged code and closed issues do not constitute approval.

## Exception contract

Every exception must include:

- owner and approving authority;
- exact capability, profile, contract or evidence affected;
- rationale and bounded risk;
- compensating control;
- expiry date or release;
- reconciliation issue and required proof.

An expired exception blocks the next gate automatically until renewed or resolved.

## Gate sequence

- M0.5 Knowledge Foundation: #61 — Approved.
- Runtime Foundation implementation readiness: #54 — currently Rework required.
- M1–M8: governed by their release epic and the capability/release matrix.
- RC: all M1–M8 exit evidence, compatibility matrix and UC-BoK drift disposition accepted.
- 1.0: signed release artifacts and the complete accepted evidence bundle.

The critical path and release dependencies are maintained in `uc-rust-1.0-scope-and-traceability.md`.