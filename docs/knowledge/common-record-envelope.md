# Common Architecture Record Envelope

- Status: Complete
- Governing issue: #56
- Governing decision: ADR-0022
- Lifecycle model: `docs/architecture/record-lifecycle.md`
- Relationship model: `docs/architecture/relationship-model.md`
- Authority model: `docs/architecture/authority-boundaries.md`
- Schema: `schemas/architecture-record-envelope.schema.json`

## Purpose

The common envelope defines metadata shared by every normative architecture record. It provides stable identity, ownership, lifecycle, provenance, traceability, freshness and evidence semantics while allowing each record family to own a specialized body.

## Canonical source representation

The authoritative source representation is Markdown with machine-validatable YAML front matter:

- YAML front matter contains the common envelope and typed relations;
- the Markdown body contains type-specific normative content;
- the body must not redefine envelope semantics;
- generated JSON, indexes and graph projections are derived artifacts;
- formatting normalization does not require a `content_version` change;
- schema validation rejects unknown lifecycle states and uncontrolled relation types.

## Required fields

| Field | Meaning |
|---|---|
| `id` | Stable globally resolvable record identifier within its namespace. |
| `type` | Controlled record-family code such as `CR` or `QAR`. |
| `schema_version` | Version of the machine-validatable record schema. |
| `content_version` | Version of the governed meaning of this record. |
| `title` | Concise human-readable name. |
| `summary` | Normative intent in a few sentences. |
| `status` | Record lifecycle state. |
| `owners` | Accountable people or roles, not merely contributors. |
| `created_at` | Original creation date. |
| `updated_at` | Date of the latest semantic update. |
| `scope` | Concern for which this record is authoritative. |
| `provenance` | Originating issue, session, source or external authority. |
| `relations` | Typed, directional references governed by the controlled relationship model. |
| `review` | Required review ownership and current disposition. |

## Conditionally required fields

| Field | Required when |
|---|---|
| `non_goals` | The record could otherwise be interpreted too broadly. |
| `evidence` | Evidence is normatively referenced by the record. |
| `review.reviewed_at` | A review disposition exists. |
| `review.next_review_at` | Freshness is time-sensitive. |
| `supersedes` | This record replaces another accepted record. |
| `superseded_by` | Status is `Superseded`. |
| `deprecation` | Status is `Deprecated`. |
| `waivers` | A known non-conformance is temporarily accepted. |

## Identity

Canonical identity format:

```text
<namespace>:<local-id>
```

Examples:

```text
uc-rust:ADR-0022
uc-rust:CR-0001
uc-rust:RRR-0001
uc-bok:CAP-COMMERCE-BASKET
yukh:META-RELATION-0001
ebd:ER-0007
```

The namespace identifies the authoritative repository or registry. Local identifiers are immutable and never reused. Titles, aliases and paths aid discovery but are not identity. Cross-repository references preserve the owning identifier. Yukh may index and reconcile identifiers without becoming their authority.

## Version dimensions

`schema_version` and `content_version` are separate:

- `schema_version` changes when representation or validation rules change;
- `content_version` changes when governed meaning changes.

Editorial fixes update `updated_at` but need not increment `content_version`.

## Lifecycle

`status` uses only the lifecycle states defined in `docs/architecture/record-lifecycle.md`: `Draft`, `Proposed`, `Accepted`, `Deprecated`, `Superseded`, `Rejected` and `Withdrawn`.

Implementation progress, verification freshness, release inclusion and deployment state are orthogonal and must not be encoded as lifecycle values.

## Owners and reviewers

Owners are accountable for correctness and freshness. Accountable roles may be used before named people are assigned. Reviewers provide independent disposition. Acceptance records who made the disposition and under which governance rule. Absence of objection is never acceptance.

## Scope and non-goals

`scope` states what the record owns. `non_goals` protects adjacent concerns from accidental capture. A record does not become authoritative merely because it links to or explains another concern.

## Provenance

Initial source kinds are `issue`, `session`, `uc-bok`, `external-standard`, `incident`, `experiment` and `migration`. Provenance identifies origin, not correctness or authority transfer.

## Relations

Each relation contains:

- required `type`;
- required canonical `target` identifier;
- optional `scope`;
- optional `valid_from` and `valid_until`;
- optional provenance;
- `confidence` only for inferred, non-normative projection edges.

Normative relations are written only in the authoritative source record. Hand-maintained inverse edges are forbidden except for lifecycle metadata explicitly required by the lifecycle model. The controlled vocabulary, source/target matrix, direction, temporal rules and validator invariants are defined in `docs/architecture/relationship-model.md`.

Generic `related_to`, `links`, `see_also` and equivalent relation types are invalid.

## Evidence

Evidence references are not anonymous URLs. Each item declares kind, locator or canonical identifier, supported claim, produced date, relevant environment/fingerprint, freshness or expiry and result/disposition.

Initial evidence kinds include code, test, benchmark, trace, report, deployment, operational observation, audit and review. Evidence does not become normative merely by being referenced or bundled by a release, and evidence alone cannot cause a lifecycle transition.

## Review metadata

A review block declares required reviewer roles, current reviewers, disposition, review date, next review date or trigger, unresolved objections and bounded exceptions.

## Forbidden ambiguities

- `status` does not mean implementation progress;
- `version` does not combine schema and content versions;
- `owner` does not mean author;
- `evidence` does not imply verification automatically;
- `release` does not imply lifecycle acceptance;
- generic `links` do not replace typed relations;
- inverse relation names do not replace canonical relation direction;
- a relation does not prove conformance, deployment or acceptance;
- a local copy does not replace an unresolved external authority identifier.

## Completion statement

The envelope is complete for 1.0 because its fields, lifecycle semantics, relation semantics, authority rules and evidence separation are explicit, represented by a versioned schema and exercised by the canonical CR-0001 and RRR-0001 records.