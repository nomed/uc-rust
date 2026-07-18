# UC Rust 1.0 Record Validation Contract

- Status: Implemented
- Governing issue: #65
- Depends on: #56, #62, #63, #64

## Canonical input

A normative record is one UTF-8 Markdown file beginning at byte zero with YAML front matter delimited by `---` lines. The front matter must decode to one mapping. The remainder is the type-specific Markdown body.

The validator does not obtain canonical meaning from generated JSON, Yukh, a graph database, network resolution or repository issues. External identifiers are checked syntactically; resolution is a separate authority-boundary concern.

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

## CLI

```bash
python -m pip install -r requirements-dev.txt
python scripts/validate_records.py docs/knowledge/records
python scripts/validate_records.py path/to/one-record.md
```

Exit codes:

- `0`: every selected record is valid;
- `1`: one or more actionable diagnostics were produced;
- `2`: no records were selected.

Each diagnostic identifies `file:field`, a stable rule name and a human-readable explanation.

## Fixtures and CI

`tests/test_record_validator.py` proves that CR-0001 and RRR-0001 pass and that `tests/fixtures/records/invalid-unknown-relation.md` fails specifically under `relation-type`.

`.github/workflows/architecture-records.yml` runs canonical validation and fixture assertions on relevant pull requests and pushes to `main`.

## Extension rule

A schema change must increment `schema_version` when existing version-1 files would change interpretation or validity. Adding a record family also requires the taxonomy extension process from ADR-0022 before its code, body profile and relation constraints may be accepted.
