# UC Rust 1.0 Controlled Relationship Model

- Status: Complete
- Governing issue: #64
- Depends on: #62
- Related: #56, #63, #65, ADR-0022

## Purpose

This document defines the controlled, directional relationship vocabulary for UC Rust 1.0 Normative Records. Relations express architecture claims. They are not generic hyperlinks, tags or inferred graph edges.

## Relation object

A relation is represented in the source record envelope:

```yaml
relations:
  - type: implements
    target: uc-rust:CR-0001
    scope: basket mutation semantics
    valid_from: 2026-07-18
    provenance:
      kind: issue
      locator: https://github.com/nomed/uc-rust/issues/64
```

Required fields:

- `type` — controlled relation code;
- `target` — canonical target identifier.

Optional fields:

- `scope` — bounded meaning when the relation does not cover the complete target;
- `valid_from` and `valid_until` — temporal validity of the assertion;
- `provenance` — origin of the assertion;
- `confidence` — permitted only on inferred, non-normative projection edges.

The source record is implicit: it is the record containing the relation.

## Authority

The namespace authority owning the relation source owns the normative assertion.

An incoming relation:

- does not transfer authority to the source;
- does not imply endorsement by the target authority;
- does not allow the source to redefine target meaning;
- may be rejected as unresolved when its target cannot be resolved.

A relation emitted by Yukh or another projection is inferred and non-normative until accepted into the authoritative source record.

## Controlled vocabulary

### `references`

The source uses the target as context or input without claiming realization, conformance, dependency or replacement.

- Source: any Normative Record.
- Target: Concept, Normative Record, external standard or other canonical authority identifier.
- Cardinality: many-to-many.
- Inverse: none normative.
- Constraint: cannot substitute for a more precise relation.

### `realizes_concept`

The source record provides a governed UC Rust realization of an externally owned concept.

- Source: CR, RRR, QAR, ER, SR, IR or DR.
- Target: externally owned Concept identifier.
- Cardinality: many source records may realize one concept; one source may realize multiple concepts only with explicit `scope`.
- Inverse: `realized_by` may be derived by projections but is not written normatively in UC Rust.

### `depends_on`

The source's normative meaning cannot be satisfied without the target's normative contract.

- Source: any Normative Record.
- Target: Normative Record or external normative authority.
- Cardinality: many-to-many.
- Inverse: `required_by` is derived only.
- Constraint: dependency cycles are invalid unless every edge is explicitly scoped and an ADR justifies the cycle.

### `constrains`

The source imposes a normative bound on the target.

- Source: ADR, QAR, ER, SR, IR, DR or RR.
- Target: Normative Record.
- Cardinality: many-to-many.
- Inverse: `constrained_by` is derived only.
- Constraint: the source must own the constraint semantics; it cannot silently redefine target scope.

### `implements`

The source normative record defines the implementation contract for all or part of the target record.

- Source: RRR, IR, DR or an implementation-focused ADR.
- Target: CR, RRR, IR or DR.
- Cardinality: many-to-many; partial implementation requires `scope`.
- Inverse: `implemented_by` is derived only.
- Constraint: this relation does not prove implementation conformance; conformance requires evidence.

### `exposes`

The source makes part of its governed behavior available through the target interface contract.

- Source: CR or RRR.
- Target: IR.
- Cardinality: many-to-many; `scope` is required when the interface exposes only part of the source.
- Inverse: `exposes_for` is not stored; projections may derive `interface_for`.

### `uses_data`

The source relies on the target data contract or governed data semantics.

- Source: CR, RRR, IR, QAR, SR, ER or RR.
- Target: DR.
- Cardinality: many-to-many.
- Inverse: `used_by` is derived only.
- Constraint: read/write/ownership semantics belong in `scope` or the DR body, not in new ad-hoc relation names.

### `satisfies`

The source is designed to satisfy the normative target concern.

- Source: CR, RRR, IR, DR, ADR or RR.
- Target: QAR, SR or ER.
- Cardinality: many-to-many.
- Inverse: `satisfied_by` is derived only.
- Constraint: this is a design claim, not verification. Evidence determines demonstrated satisfaction.

### `governed_by`

The source is governed by the target decision or policy record.

- Source: any Normative Record except the target itself.
- Target: ADR or accepted external governance identifier.
- Cardinality: many-to-many.
- Inverse: `governs` is derived only.
- Constraint: use `constrains` when the source record itself owns a specific bound on the target.

### `included_in`

The source is normatively included in the target release composition.

- Source: ADR, RFC, CR, RRR, QAR, ER, SR, IR or DR.
- Target: RR.
- Cardinality: many-to-many.
- Inverse: `includes` is derived only.
- Constraint: inclusion does not imply lifecycle acceptance, implementation, verification or deployment.

### `supersedes`

The source replaces all or an explicitly scoped part of the target's accepted normative meaning.

- Source: any Accepted Normative Record.
- Target: Accepted or Deprecated record of a semantically compatible family.
- Cardinality: one-to-many only when replacement scope is explicit and non-overlapping.
- Inverse: `superseded_by` is materialized in lifecycle metadata for the target and may also be derived.
- Constraint: target must transition to Superseded through the lifecycle model.

### `deprecates`

The source provides the normative reason or preferred path that causes the target to be discouraged for new work.

- Source: ADR, RR or accepted successor record.
- Target: Accepted record.
- Cardinality: many-to-many with explicit scope.
- Inverse: none stored.
- Constraint: target must transition to Deprecated through the lifecycle model; the relation alone does not change status.

### `conflicts_with`

The source has an explicitly identified normative incompatibility with the target.

- Source: any Normative Record.
- Target: Normative Record or external normative authority.
- Cardinality: many-to-many.
- Direction: assertion authority remains with the source, although the incompatibility may be semantically symmetric.
- Constraint: rationale and conflict scope are mandatory. A projection may show the reverse edge but cannot claim mutual acknowledgement.

### `waives`

The source grants a bounded exception to the target requirement.

- Source: ADR or RR.
- Target: QAR, ER, SR, IR, DR, CR or RRR.
- Cardinality: many-to-many.
- Constraint: `scope`, `valid_until`, accountable approver and risk rationale are mandatory. Permanent waivers are forbidden; replace the target requirement instead.

## Source and target matrix

| Relation | Allowed source families | Allowed target kinds |
|---|---|---|
| `references` | Any | Concept, Record, external authority |
| `realizes_concept` | CR, RRR, QAR, ER, SR, IR, DR | Concept |
| `depends_on` | Any | Record, external normative authority |
| `constrains` | ADR, QAR, ER, SR, IR, DR, RR | Record |
| `implements` | RRR, IR, DR, implementation ADR | CR, RRR, IR, DR |
| `exposes` | CR, RRR | IR |
| `uses_data` | CR, RRR, IR, QAR, SR, ER, RR | DR |
| `satisfies` | CR, RRR, IR, DR, ADR, RR | QAR, SR, ER |
| `governed_by` | Any non-self record | ADR, external governance |
| `included_in` | All non-RR baseline families | RR |
| `supersedes` | Accepted Record | compatible Record |
| `deprecates` | ADR, RR, accepted successor | Accepted Record |
| `conflicts_with` | Any | Record, external normative authority |
| `waives` | ADR, RR | governed requirement record |

## Direction and inverse policy

Normative relations are stored only in the source record. Inverse edges are projections unless lifecycle metadata explicitly requires a reciprocal field.

This avoids dual-write inconsistency. A validator must reject hand-authored inverse relation codes such as:

- `implemented_by`;
- `required_by`;
- `constrained_by`;
- `satisfied_by`;
- `realized_by`;
- `includes`.

The canonical direction is chosen according to the record making the architecture claim.

## Temporal validity

- Absence of `valid_from` means validity begins with the content version containing the relation.
- Absence of `valid_until` means no planned expiry.
- `valid_until` must be later than `valid_from`.
- Expired relations remain in version history but are not active in current projections.
- A relation's expiry does not mutate either endpoint's lifecycle.
- `waives` always requires `valid_until`.

## Provenance

Normative relations inherit the source record authority but should identify provenance when introduced by migration, external reconciliation, incident, experiment or explicit governance decision.

Provenance explains where the assertion originated. It does not determine correctness.

## Normative versus inferred relations

Normative relation:

- appears in the authoritative source record;
- uses a controlled relation type;
- satisfies source/target constraints;
- follows the source record lifecycle and content version.

Inferred relation:

- exists only in a projection;
- declares inference method and confidence;
- cannot drive acceptance, supersession, release inclusion or waiver;
- becomes normative only through an explicit source-record change and review.

## Forbidden patterns

The following are invalid:

- `related_to`, `link`, `association`, `see_also` or other generic relation types;
- file paths, titles or URLs used as record identity when a canonical identifier exists;
- storing both normative direction and a hand-maintained inverse;
- using `implements` as evidence of conformance;
- using `included_in` as evidence of deployment or acceptance;
- using `references` where `depends_on`, `constrains`, `implements`, `satisfies` or another precise relation applies;
- relation targets embedded only in free text;
- inferred relations written as normative without accountable review;
- cross-namespace target copies that replace the owning identifier;
- self-relations, except a validator-approved migration alias outside the normative relation set;
- unresolved target identifiers silently ignored.

## Canonical examples

### Capability and concept

```yaml
- type: realizes_concept
  target: uc-bok:CAP-COMMERCE-BASKET
```

### Runtime responsibility and capability

```yaml
- type: implements
  target: uc-rust:CR-0001
  scope: operation invocation mechanics
```

### Interface and data

```yaml
- type: uses_data
  target: uc-rust:DR-0001
  scope: basket command and result contracts
```

### Quality, security and economics

```yaml
- type: satisfies
  target: uc-rust:QAR-0001
- type: satisfies
  target: uc-rust:SR-0001
- type: satisfies
  target: uc-rust:ER-0001
```

### Release composition

```yaml
- type: included_in
  target: uc-rust:RR-0001
```

## Required traces by record family

- CR: concept realization, runtime/interface/data implementation, quality/security/economic constraints and release inclusion.
- RRR: implemented capability/responsibility, used interfaces/data, satisfied QAR/SR/ER and release inclusion.
- IR: exposed capability/responsibility, data use, compatibility decisions and release inclusion.
- DR: capability/interface use, security/quality/economic constraints and release inclusion.
- QAR: records it constrains and evidence references in its own evidence block.
- SR: records it constrains and bounded waivers where applicable.
- ER: records it constrains and release decisions affected by economic gates.
- RR: governed records included in the release are represented by incoming `included_in` edges; RR may `constrain`, `waive`, `deprecate` or `supersede` where release governance requires it.

## Validator invariants for #65

A validator must detect:

1. unknown relation types;
2. missing target identifiers;
3. invalid source-family and target-kind combinations;
4. self-relations;
5. hand-authored inverse codes;
6. missing `scope` where partiality or ambiguity requires it;
7. missing expiry and approval metadata for `waives`;
8. invalid temporal intervals;
9. dependency cycles lacking an ADR justification;
10. `supersedes` targets without compatible lifecycle transitions;
11. unresolved cross-namespace identifiers as explicit findings;
12. confidence on normative edges or absent confidence/method on inferred edges.

## Completion statement

The vocabulary is intentionally minimal. New relation types require an ADR demonstrating that no existing type plus `scope` can express the claim safely, and must define source/target constraints, direction, inverse policy, temporal semantics and validator impact.
