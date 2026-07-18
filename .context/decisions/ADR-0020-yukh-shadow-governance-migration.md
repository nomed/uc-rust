# ADR-0020 — Migrate GitHub governance to Yukh through shadow mode

- Status: Accepted
- Date: 2026-07-18
- Related: nomed/uc-rust#20
- External: nomed/yukh

## Context

UC Rust currently embeds scripts that synchronize repository metadata, GitHub Project fields, issue relationships and roadmap metadata. The generic responsibility now belongs in Yukh, a dedicated model-driven project-intelligence engine.

Removing the embedded implementation immediately would create operational risk because Yukh has not yet demonstrated behavioral equivalence against UC Rust.

## Decision

UC Rust will migrate to Yukh through a controlled shadow process.

1. Existing UC Rust manifests and embedded scripts remain authoritative for apply operations during the shadow phase.
2. No new generic governance behavior is added to the embedded scripts except defect fixes required to preserve the baseline.
3. Yukh consumes equivalent UC Rust fixtures and computes desired-versus-actual drift without mutating UC Rust.
4. Differences are classified explicitly as defects, semantic mismatches or intentional divergences.
5. Yukh may become the apply engine only after zero unexplained drift, idempotency evidence, failure-safety evidence and acceptance in both repositories.
6. UC Rust will consume a pinned Yukh release or action after the migration gate.
7. Legacy scripts are removed only after rollback has been demonstrated.

## Consequences

- Governance development continues in Yukh rather than being duplicated in UC Rust.
- UC Rust implementation work, including Economics by Design, continues in parallel.
- The migration is evidence-driven and reversible.
