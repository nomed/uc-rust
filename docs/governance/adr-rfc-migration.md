# ADR and RFC Migration to the 1.0 Record Envelope

- Status: Accepted implementation policy
- Governing issue: #66
- Depends on: #56, #63, #64, #65
- Effective date: 2026-07-18

## Purpose

This policy defines a safe, incremental migration of pre-1.0 ADR and RFC files to the common architecture-record envelope. It preserves historical meaning, identifiers, decisions and review evidence while ensuring that all newly created records are validatable.

## Scope

The migration population is every Markdown file present under `.context/decisions` or `.context/rfcs` at the effective date. Files created after the effective date are not legacy records and must conform to schema version 1 immediately.

## Migration dispositions

Every legacy file receives exactly one disposition:

| Disposition | Meaning |
|---|---|
| `grandfathered` | Historical record remains authoritative in its existing representation. Meaning is frozen; metadata may be supplied through the migration registry. |
| `backfill-in-place` | Record is not Accepted and may receive YAML front matter without changing its normative meaning. |
| `canonicalize-on-change` | Record remains grandfathered until a normative amendment is proposed; the proposal must use the 1.0 envelope. |
| `supersede` | Historical record remains immutable and a new conforming record replaces all or part of it. |
| `collision-remap` | A duplicate legacy local identifier is assigned the next unused canonical identifier; the conflicting path/title is retained as an alias. |
| `exclude-non-record` | File does not represent an ADR or RFC despite its location and is excluded with a rationale. |

Disposition is calculated by the rules below and may be overridden only by an explicit entry in `governance/record-migration-registry.yaml`.

## Deterministic classification

1. A record whose normalized status is `Accepted`, `Deprecated`, `Superseded`, `Rejected` or `Withdrawn` is `grandfathered`.
2. A `Draft` or `Proposed` record with no identifier collision is `backfill-in-place`.
3. A record whose identifier duplicates an earlier reservation is `collision-remap` unless human evidence establishes a different reservation order.
4. A grandfathered record requiring normative change is `canonicalize-on-change` and returns to `Proposed` under the lifecycle model, or is replaced by a new record using `supersedes`.
5. A file that cannot yield an identifier, title and status is reported as `manual-review`; this is a validation failure, not an accepted disposition.

Reservation order is determined by the earliest Git creation evidence when available. When that evidence is unavailable, the migration registry is authoritative.

## Grandfathering policy

Grandfathering is representation compatibility, not semantic re-acceptance.

A grandfathered record:

- retains its original local identifier and canonical identifier `uc-rust:<local-id>`;
- retains its original file and accepted body unchanged;
- may use registry-supplied metadata for schema projection and indexing;
- is not required to be rewritten merely to satisfy the new envelope;
- must not be silently edited to alter accepted meaning;
- must be superseded or formally reopened for normative change.

The validator may report grandfathered records separately, but it must not treat their legacy representation as a new schema-valid record.

## Metadata backfill

For immutable historical records, missing envelope fields are supplied in the migration registry rather than inserted into the accepted file. Allowed backfill fields are:

- canonical `id` and `type`;
- normalized lifecycle `status`;
- title and summary extracted without interpretation;
- owners/reviewers when explicitly evidenced;
- creation and update dates;
- provenance;
- aliases and previous paths;
- typed relations that restate explicit historical references;
- migration disposition and rationale.

Backfill must not invent scope, rationale, acceptance, evidence, relations or owners. Unknown values remain explicit findings.

Draft and Proposed records may be converted in place to schema version 1 when the conversion preserves their meaning. The conversion itself does not accept the record.

## Identifier and alias preservation

Canonical identity is `uc-rust:<local-id>`. Local identifiers are never reused, including identifiers of Rejected, Withdrawn, Deprecated or Superseded records.

Paths, old headings and informal references are aliases only. The migration registry records aliases as locators and never treats them as canonical identity.

### RFC-0001 collision

Two draft files historically claimed `RFC-0001`. The reservation is resolved as follows:

- `.context/rfcs/RFC-0001-agentic-operating-system.md` retains `uc-rust:RFC-0001`;
- `.context/rfcs/RFC-0001-runtime-foundation.md` is remapped to `uc-rust:RFC-0002`;
- the old runtime-foundation path and heading are retained as legacy aliases;
- `RFC-0001` is never interpreted as referring to the runtime-foundation proposal after this policy takes effect.

This resolves an invalid duplicate reservation; it does not rewrite an accepted decision because both files were Draft.

## Supersession and deprecation

Migration does not infer supersession from chronology or topic similarity.

A supersession migration requires:

- an Accepted successor;
- an explicit `supersedes` relation from successor to predecessor;
- an explicit `superseded_by` relation or registry projection for the predecessor;
- replacement scope when supersession is partial;
- an attributable approval event.

Deprecation requires an explicit rationale and migration guidance. A Deprecated record remains valid for historical or compatibility use until superseded or retired by policy.

## New-record rule

After the effective date:

- ADRs and RFCs use the schema-version-1 YAML envelope;
- their IDs are allocated from the migration registry's reserved set;
- all relations use the controlled vocabulary;
- lifecycle changes follow `docs/architecture/record-lifecycle.md`;
- acceptance remains an attributable human act;
- new duplicate IDs fail CI.

## Automation contract

`scripts/inventory_legacy_records.py` scans the legacy directories and emits a deterministic inventory. It must:

- extract local ID, title and status;
- normalize legacy status spelling and case;
- detect duplicate IDs and malformed records;
- apply registry overrides;
- reserve every discovered identifier;
- produce actionable diagnostics;
- require no network, Yukh or external service.

The inventory is evidence for migration planning, not a lifecycle transition.

## Completion criteria

Issue #66 is complete when:

1. all legacy ADR/RFC files are covered by deterministic classification rules;
2. the known RFC identifier collision is resolved;
3. accepted history is protected from semantic backfill;
4. future templates conform to the 1.0 envelope;
5. identifier reservation and alias rules are machine-checkable;
6. migration can run locally without external infrastructure.
