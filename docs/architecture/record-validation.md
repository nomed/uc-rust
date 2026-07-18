# UC Rust 1.0 Record Validation Contract

- Status: Implemented
- Governing issues: #59, #65
- Depends on: #56, #57, #58, #62, #63, #64

## Canonical input

A normative record is one UTF-8 Markdown file beginning at byte zero with YAML front matter delimited by `---` lines. The front matter must decode to one mapping. The remainder is the type-specific Markdown body.

The validator does not obtain canonical meaning from generated JSON, Yukh, a graph database, network resolution or repository issues. External identifiers are checked against the namespace-authority registry without requiring network access.

Identity, filenames, schema compatibility and content-version rules are defined in `docs/knowledge/record-identifiers-and-versioning.md`.

## Schema version 1

`schemas/architecture-record-envelope.schema.json` is the declarative common-envelope schema. The executable validator additionally enforces rules that require cross-field or source/target reasoning:

- identifier family equals `type`;
- lifecycle status and review disposition are consistent;
- final review dispositions have an attributable review date;
- superseded and deprecated records carry required metadata;
- relation types belong to the controlled vocabulary;
- self-relations, duplicate relations and inferred canonical edges are rejected;
- relation-specific target and family constraints are enforced;
- each record family contains its minimum normative body sections.

All 1.0 record families use the common envelope and a body profile:

| Type | Minimum body sections |
|---|---|
| ADR | Context, Decision, Consequences |
| RFC | Context, Proposal |
| CR | Purpose, Invariants, Authority |
| RRR | Responsibility, Boundary, Invariants |
| QAR | Quality scenario, Measure |
| ER | Economic scope, Economic constraints |
| SR | Security scope, Threats, Controls |
| IR | Interface, Contract |
| DR | Data scope, Invariants |
| RR | Release scope, Entry criteria, Exit criteria |

These are minimum structural contracts, not complete semantic substitutes for human review.

## Repository graph integrity

`scripts/validate_record_graph.py` validates repository-wide invariants that cannot be checked one file at a time:

- duplicate canonical identifiers;
- unresolved local `uc-rust:` references;
- unknown external authorities;
- duplicate and self-referential edges;
- forbidden hand-authored inverse relations;
- cycles in `depends_on` and `supersedes`;
- isolated architecture records;
- Accepted records without accountable ownership.

External unavailability remains a resolution finding and never causes a local authority copy.

## Historical transition validation

`scripts/validate_record_changes.py` compares changed records with a Git base reference. It rejects:

- mutation of `id`, `type` or `created_at`;
- lifecycle transitions outside the accepted state machine;
- new records beginning in a terminal or accepted state;
- semantic envelope changes without a content-version increment;
- content-version regression;
- Accepted state without accepted review metadata;
- deletion of an Accepted canonical record.

Lifecycle-only changes remain orthogonal to content version: acceptance, deprecation and supersession do not by themselves change governed meaning.

## CLI

```bash
python -m pip install -r requirements-dev.txt
python scripts/validate_records.py docs/knowledge/records
python scripts/validate_record_graph.py docs/knowledge/records
python scripts/validate_record_changes.py --base-ref origin/main docs/knowledge/records
python scripts/validate_records.py path/to/one-record.md
```

Exit codes:

- `0`: every selected record or transition is valid;
- `1`: one or more actionable diagnostics were produced;
- `2`: no records were selected where the command distinguishes that case.

Each diagnostic identifies `file:field`, a stable rule name and a human-readable explanation.

## Fixtures and CI

`tests/test_record_validator.py` proves that CR-0001 and RRR-0001 pass and that deliberately invalid fixtures fail under expected stable rule names.

`.github/workflows/architecture-records.yml` runs:

1. canonical envelope/body validation;
2. repository graph-integrity validation;
3. pull-request lifecycle-transition validation against the base branch;
4. legacy ADR/RFC migration inventory validation;
5. validator fixture assertions.

The checkout uses complete Git history where transition comparison is required.

## Schema compatibility and migration

A schema change must increment `schema_version` when existing version-1 files would change interpretation or validity. Validators must continue supporting schema versions still present in canonical records, or fail explicitly as unsupported.

A schema migration requires deterministic transformation rules, compatibility impact, migration fixtures and registry updates. A change to taxonomy or governed meaning additionally requires an ADR. Existing accepted history is never rewritten merely to adopt a newer representation.

## Extension rule

Adding a record family requires the taxonomy extension process from ADR-0022 before its code, body profile and relation constraints may be accepted. New relation types require the relationship-model extension rule. Projection infrastructure remains optional and non-authoritative.