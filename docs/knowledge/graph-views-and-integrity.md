# Architecture Knowledge Graph Views and Integrity

- Status: Complete
- Governing issue: #58
- Depends on: #56, #64, #67

## Purpose

This document defines the deterministic graph projections and repository-level integrity checks built from authoritative UC Rust records. The graph is derived: records and their declared relations remain authoritative in Git.

## Node and edge contract

Each normative record becomes one node keyed by its canonical `id`. Each entry in `relations` becomes one directed edge:

```text
(source record id, relation type, target id, optional scope)
```

The edge authority is the source record. Reverse edges, transitive closure, confidence scores and impact sets are projections and are never written back as normative facts without review.

## Resolution classes

Targets are classified as:

- `local-resolved` — the canonical target exists in the scanned UC Rust record set;
- `external-declared` — the namespace exists in the authority registry and resolution is delegated to that authority;
- `external-unavailable` — the authority is known but cannot currently be queried;
- `unresolved-local` — a `uc-rust:` target is missing from the scanned authoritative set;
- `unknown-authority` — the target namespace has no declared authority.

`unresolved-local` and `unknown-authority` are errors. External unavailability is an explicit finding and must never cause creation of a local substitute.

## Integrity invariants

1. Record identifiers are unique across the scanned authoritative set.
2. Every `uc-rust:` relation target resolves locally.
3. Every external namespace is declared in `governance/namespace-authorities.yaml`.
4. `supersedes` is acyclic.
5. `depends_on` is acyclic unless every edge in the cycle has explicit scope and an accepted ADR is referenced as cycle justification.
6. A record cannot relate to itself.
7. Duplicate `(source, type, target, scope)` edges are invalid.
8. Hand-authored inverse relation names are invalid.
9. Expired edges remain historical but are excluded from current views.
10. Inferred edges are projection-only and cannot satisfy required normative traces.
11. Accepted records without owners are orphaned and cannot satisfy a release gate.
12. A normative record with no incoming or outgoing architecture relation is reported as isolated; isolation is an error for CR, RRR, IR, DR, QAR, SR and ER, and a warning for ADR, RFC and RR.

## Required graph views

### Decision impact

**Input:** ADR identifier.

**Traversal:** outgoing `constrains`, `governed_by`, `supersedes`, `deprecates`, plus incoming `governed_by` and `depends_on` projections.

**Answer:** records whose normative interpretation, implementation or release disposition may change when the decision changes.

### Capability realization

**Input:** CR identifier or UC-BoK concept identifier.

**Traversal:** `realizes_concept`, incoming `implements`, `exposes`, `uses_data`, `satisfies`.

**Answer:** capability meaning, runtime responsibilities, interfaces, data contracts and governing quality/security/economic constraints.

### Verification coverage

**Input:** any normative record.

**Traversal:** the record evidence block plus records connected by `satisfies`, `constrains` and `included_in`.

**Answer:** evidence that supports claims, its freshness, environment and gaps. Relations alone never count as verification evidence.

### Release composition

**Input:** RR identifier.

**Traversal:** incoming `included_in`, outgoing `waives`, `deprecates`, `supersedes` and `constrains`.

**Answer:** included normative scope, accepted exceptions, compatibility impact and missing prerequisites. Inclusion does not imply implementation or verification.

### Runtime dependency

**Input:** RRR identifier.

**Traversal:** `depends_on`, `implements`, `uses_data`, `exposes` and their derived incoming views.

**Answer:** prerequisites, implemented responsibilities, interfaces and data dependencies, with cycles surfaced explicitly.

### UC-BoK traceability

**Input:** `uc-bok:` concept identifier.

**Traversal:** incoming `realizes_concept` and the downstream capability realization view.

**Answer:** all UC Rust governed realizations without copying UC-BoK authority.

### Economic attribution

**Input:** ER identifier or governed record.

**Traversal:** `constrains`, `satisfies`, `depends_on`, `included_in`.

**Answer:** which capabilities/runtime responsibilities are economically constrained and which releases consume the economic gate.

### Stale and orphaned knowledge

**Input:** repository or release scope.

**Checks:** freshness projection, missing owner, unresolved target, isolated record, expired waiver, missing required trace.

**Answer:** actionable findings with record, field, rule and remediation owner.

## Query result contract

Every projection query returns:

- query name and input identifier;
- authoritative nodes and declared edges used;
- derived edges clearly marked `inferred`;
- resolution status for each external target;
- validity timestamp;
- diagnostics and incomplete-result reasons.

A query must not present an incomplete graph as complete when an authority is unavailable.

## Minimum trace expectations

- CR: at least one `realizes_concept` or explicit rationale for a platform-only capability, plus a runtime/interface path before release inclusion.
- RRR: at least one `implements` target.
- IR: at least one incoming or outgoing capability/runtime trace.
- DR: at least one `uses_data` consumer or explicit foundational-data rationale.
- QAR, SR, ER: at least one constrained or satisfied subject before release gating.
- RR: at least one incoming `included_in` edge before release disposition.

These expectations are graph-level checks and do not change lifecycle automatically.

## Canonical questions

The model can answer without free-text interpretation:

- **What does ADR-X affect?** Decision impact view.
- **What realizes capability X?** Capability realization view.
- **What verifies record X?** Evidence coverage view, not an `implements` edge.
- **What enters release X?** Incoming `included_in` edges.
- **What is stale, isolated or unresolved?** Integrity/freshness view.

## Tooling boundary

`scripts/validate_record_graph.py` performs repository-local deterministic checks without Yukh. Yukh may provide richer cross-repository resolution, traversal and visualization, but it must preserve the same authority and result contract.