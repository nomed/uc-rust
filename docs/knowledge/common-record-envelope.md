# Common Architecture Record Envelope

- Status: Draft
- Governing issue: #56
- Governing decision: ADR-0022

## Purpose

The common envelope defines metadata shared by every normative architecture record. It provides stable identity, ownership, lifecycle, provenance, traceability, freshness and evidence semantics while allowing each record family to own a specialized body.

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
| `owners` | Accountable maintainers, not merely contributors. |
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
uc-rust:<TYPE>-<NNNN>
```

Examples:

```text
uc-rust:ADR-0022
uc-rust:CR-0001
uc-rust:RRR-0001
```

Cross-repository references use the owning namespace:

```text
uc-bok:CAP-COMMERCE-BASKET
yukh:META-RELATION-0001
ebd:ER-0007
```

The numeric suffix is immutable and never reused. Renaming a title does not change identity.

## Version dimensions

`schema_version` and `content_version` are separate:

- `schema_version` changes when representation or validation rules change;
- `content_version` changes when governed meaning changes.

Editorial fixes that do not alter meaning update `updated_at` but need not increment `content_version`.

## Owners and reviewers

Owners are accountable for correctness and freshness. Reviewers provide required independent disposition. One person may contribute to both roles, but acceptance must record who made the disposition and under which governance rule.

## Scope and non-goals

`scope` states what the record owns. `non_goals` protects adjacent concerns from accidental capture. A record must not become authoritative merely because it contains a link or explanatory paragraph about another concern.

## Provenance

Provenance is typed. Initial source kinds are:

- `issue`;
- `session`;
- `uc-bok`;
- `external-standard`;
- `incident`;
- `experiment`;
- `migration`.

A provenance item identifies origin, not correctness.

## Relations

Each relation contains:

- `type`;
- `target`;
- optional `scope`;
- optional `valid_from` and `valid_until`;
- optional provenance;
- optional confidence for inferred, non-normative edges.

Normative edges may not depend solely on inferred confidence.

## Evidence

Evidence references do not live as anonymous URLs. Each evidence item declares:

- evidence kind;
- locator;
- what claim it supports;
- produced date;
- environment/fingerprint where relevant;
- freshness or expiry rule;
- result/disposition.

Initial evidence kinds include code, test, benchmark, trace, report, deployment, operational observation, audit and review.

## Review metadata

A review block declares:

- required reviewer roles;
- current reviewers;
- disposition;
- review date;
- next review date or trigger;
- unresolved objections;
- bounded exceptions.

Absence of objection is not acceptance.

## Forbidden ambiguities

The envelope must not use one field to represent multiple dimensions. In particular:

- `status` does not mean implementation progress;
- `version` does not combine schema and content versions;
- `owner` does not mean author;
- `evidence` does not imply verification automatically;
- `release` does not imply lifecycle acceptance;
- generic `links` do not replace typed relations.
