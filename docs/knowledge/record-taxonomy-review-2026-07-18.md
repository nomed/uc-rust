# Record Taxonomy Review — 2026-07-18

- Governing issue: #56
- Reviewed artifacts:
  - `.context/decisions/ADR-0022-architecture-knowledge-record-model.md`
  - `docs/knowledge/record-taxonomy.md`
  - `docs/knowledge/common-record-envelope.md`
- Review disposition: **changes requested before ADR acceptance**

## Executive disposition

The record model has a sound foundation: normative records are separated from evidence, identity is stable and namespaced, lifecycle is not conflated with implementation or verification, and taxonomy extension is constrained.

The baseline should not yet be accepted unchanged. Two taxonomy entries currently blur the model's own boundary between normative knowledge and evidence:

1. `TR — Technology Record` is described primarily as evaluation evidence and operating constraints. The current definition does not demonstrate a distinct normative responsibility that cannot be owned by an ADR, RRR, IR, SR, DR or QAR.
2. `RR — Release Record` mixes governed release intent with an evidence bundle. The normative release composition is valid, but evidence must remain referenced rather than embedded as a second semantic entity.

## Decisions recommended for issue #56

### 1. Defer `TR` from the 1.0 normative taxonomy

**Recommendation:** remove `TR` from the accepted 1.0 baseline and retain technology evaluation as structured evidence attached to an ADR or to the record whose constraints are being evaluated.

A future Technology Record may be proposed only when the extension test is satisfied with at least two concrete records and unique invariants. Typical technology selection remains an ADR.

This avoids promoting an evidence container into a normative record family.

### 2. Retain `RRR — Runtime Responsibility Record`

**Recommendation:** keep the name and abbreviation.

`Runtime Responsibility Record` expresses semantic ownership better than component-oriented alternatives. It allows a responsibility to be implemented by a crate, process, service, edge runtime or embedded module without making deployment topology part of its identity.

The definition should explicitly state that an RRR is implementation- and deployment-form independent, while still owning execution semantics and operational constraints.

### 3. Retain `RR — Release Record`, but narrow its ownership

**Recommendation:** keep `RR` as a normative record for release composition and release gates.

An RR owns:

- declared included, excluded and deferred records;
- compatibility and migration constraints;
- entry and exit gates;
- accepted exceptions and waivers;
- release disposition.

Tests, reports, deployment observations and audit results remain evidence references in the common envelope. They are not part of the Release Record's normative body.

### 4. Use canonical Markdown with YAML front matter

**Recommendation:** canonical source representation should be Markdown with a machine-validatable YAML front matter envelope.

The front matter contains the common envelope and typed relations. The Markdown body contains the type-specific normative content.

Minimum shape:

```yaml
---
id: uc-rust:CR-0001
type: CR
schema_version: 1.0.0
content_version: 0.1.0
title: Basket
summary: Governs the implementation-independent Basket capability.
status: Draft
owners:
  - nomed
created_at: 2026-07-18
updated_at: 2026-07-18
scope: Basket capability semantics and constraints.
provenance:
  - kind: issue
    locator: https://github.com/nomed/uc-rust/issues/56
relations: []
review:
  required_roles:
    - domain-architect
  disposition: pending
---
```

Rules:

- front matter is authoritative for common-envelope fields;
- the body must not redefine envelope semantics;
- generated JSON or graph projections are derived artifacts;
- ordering and formatting may be normalized without changing `content_version`;
- schema validation must reject unknown lifecycle states and uncontrolled relation types.

### 5. Identifier resolution contract

**Recommendation:** retain the canonical form `<namespace>:<local-id>`.

Examples:

- `uc-rust:CR-0001`
- `uc-bok:CAP-COMMERCE-BASKET`
- `yukh:META-RELATION-0001`
- `ebd:ER-0007`

Resolution rules:

1. the namespace identifies the authoritative repository or registry;
2. the local identifier is immutable within that namespace;
3. a title or file path is not identity;
4. cross-repository references must preserve the original canonical identifier;
5. aliases may aid discovery but may not replace canonical identity;
6. Yukh indexes and reconciles identifiers but does not become their authority.

## Required edits before ADR-0022 acceptance

1. Remove `TR` from the initial normative taxonomy or provide two concrete 1.0 records and unique invariants satisfying the extension test.
2. Clarify that RRR is independent of implementation and deployment form.
3. Narrow RR to normative release composition and gates; keep evidence external and referenced.
4. Add the Markdown/YAML-front-matter representation decision to ADR-0022 and the common envelope specification.
5. Define a controlled relation vocabulary and lifecycle model in their governing work items.
6. Create and validate `CR-0001` and `RRR-0001` against the common envelope.

## Acceptance assessment

| Area | Disposition |
|---|---|
| Normative record vs evidence boundary | Accept with RR clarification |
| Common identity and version dimensions | Accept |
| Taxonomy extension rule | Accept |
| ADR, RFC, CR, RRR, QAR, ER, SR, IR, DR | Accept as baseline candidates |
| TR | Defer / changes requested |
| RR | Accept with narrowed semantics |
| Canonical representation | YAML front matter + Markdown body |
| Canonical examples | Still required |
| Lifecycle and relation vocabularies | Still required |

ADR-0022 should remain `Proposed` until the required edits and canonical examples are complete.