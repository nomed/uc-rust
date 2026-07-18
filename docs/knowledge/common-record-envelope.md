# Common Architecture Record Envelope

- Status: Draft
- Governing issue: #56
- Governing decision: ADR-0022

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
| `relations` | Typed, directional references. |
| `review` | Required review ownership and current disposition. |

## Conditionally required fields

| Field | Required when |
|---|---|
| `non_goals` | The record could otherwise be interpreted too broadly. |
| `evidence` | Status is `Implemented`, `Verified`, `Deprecated` or `Superseded`. |
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

The namespace identifies the authoritative repository or registry. Local identifiers are immutable and never reused. Titles, aliases and paths aid discovery but are not identity. Yukh may index and reconcile identifiers without becoming their authority.

## Version dimensions

`schema_version` and `content_version` are separate:

- `schema_version` changes when representation or validation rules change;
- `content_version` changes when governed meaning changes.

Editorial fixes update `updated_at` but need not increment `content_version`.

## Owners and reviewers

Owners are accountable for correctness and freshness. Accountable roles may be used before named people are assigned. Reviewers provide independent disposition. Acceptance records who made the disposition and under which governance rule.

## Scope and non-goals

`scope` states what the record owns. `non_goals` protects adjacent concerns from accidental capture. A record does not become authoritative merely because it links to or explains another concern.

## Provenance

Initial source kinds are `issue`, `session`, `uc-bok`, `external-standard`, `incident`, `experiment` and `migration`. Provenance identifies origin, not correctness.

## Relations

Each relation contains `type`, `target`, optional `scope`, temporal validity, provenance and confidence for inferred non-normative edges. Normative edges may not depend solely on inferred confidence. Final relation types are governed by the relationship-model work item.

## Evidence

Evidence references are not anonymous URLs. Each item declares kind, locator, supported claim, produced date, relevant environment/fingerprint, freshness/expiry and result/disposition.

Initial evidence kinds include code, test, benchmark, trace, report, deployment, operational observation, audit and review. Evidence does not become normative merely by being referenced or bundled by a release.

## Review metadata

A review block declares required reviewer roles, current reviewers, disposition, review date, next review date/trigger, unresolved objections and bounded exceptions. Absence of objection is not acceptance.

## Forbidden ambiguities

- `status` does not mean implementation progress;
- `version` does not combine schema and content versions;
- `owner` does not mean author;
- `evidence` does not imply verification automatically;
- `release` does not imply lifecycle acceptance;
- generic `links` do not replace typed relations.
