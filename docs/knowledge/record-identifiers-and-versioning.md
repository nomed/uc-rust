# Architecture Record Identifiers and Versioning

- Status: Complete
- Governing issue: #59
- Governing decisions: ADR-0022, ADR-0023

## Purpose

This policy defines deterministic identity, filename, versioning, compatibility and migration rules for UC Rust architecture records.

## Canonical identity

Every normative record has exactly one immutable identifier:

```text
<namespace>:<TYPE>-<number>
```

For UC Rust records the namespace is `uc-rust`, the family is one of `ADR`, `RFC`, `CR`, `RRR`, `QAR`, `ER`, `SR`, `IR`, `DR` or `RR`, and the numeric component is zero-padded to at least four digits.

Examples:

```text
uc-rust:ADR-0023
uc-rust:CR-0001
uc-rust:RRR-0001
```

Identifiers are never reused, even after rejection, withdrawal, deprecation or supersession. Moving or renaming a file does not change identity. Historical aliases are migration metadata and never become canonical identity.

## Canonical filenames

New canonical records use:

```text
<TYPE>-<number>-<lowercase-kebab-title>.md
```

Examples:

```text
CR-0001-basket-capability.md
RRR-0001-operation-invocation.md
QAR-0001-operation-latency.md
```

The identifier in YAML front matter is authoritative. Filename/type/number disagreement is invalid. Legacy ADR/RFC filenames are governed by `docs/governance/adr-rfc-migration.md` and `governance/record-migration-registry.yaml`.

## Canonical representation

The source of truth is one UTF-8 Markdown file whose first bytes are a YAML front-matter delimiter. YAML contains the common envelope; Markdown contains the family-specific normative body. Generated JSON, graph nodes, indexes and Yukh projections are derived and disposable.

## Version dimensions

### `schema_version`

`schema_version` identifies representation and validation semantics. Version 1 is governed by `schemas/architecture-record-envelope.schema.json` and the executable validators.

A new schema version is required when an existing valid version-1 record would change interpretation or become invalid solely because validation semantics changed.

### `content_version`

`content_version` is semantic versioning for governed meaning:

- patch: clarification or bounded compatible refinement;
- minor: compatible addition of normative meaning;
- major: incompatible change that normally requires supersession or an explicitly reviewed migration.

Pure formatting, spelling and link repairs do not require a content-version increment. Any lifecycle reopening for normative amendment must increment content version before re-acceptance.

## Immutable fields

After creation these fields cannot change:

- `id`;
- record family encoded by `type`;
- `created_at`;
- canonical namespace authority.

Changing title, location or display alias never creates a new identity. Changing semantic family requires a new record and an explicit migration/supersession disposition.

## Compatibility policy

Validators must support every schema version still present in canonical records or fail with an explicit unsupported-version diagnostic. A schema version may be retired only after all canonical records have a documented migration disposition.

Schema migrations must provide:

1. old and new schema versions;
2. deterministic transformation rules;
3. compatibility impact;
4. rollback or recovery behavior;
5. migration fixtures;
6. registry updates for aliases or historical paths;
7. an ADR when governed meaning or taxonomy changes.

## Transition validation

Repository CI validates both record shape and changes over time. For an existing record it rejects:

- identifier, type or creation-date mutation;
- lifecycle transitions outside the accepted state machine;
- semantic changes without a `content_version` increment;
- content-version regression;
- Accepted status without accountable review disposition;
- deletion of an Accepted record without a supersession/migration process.

A Git commit, merge, passing check or elapsed time never performs lifecycle acceptance.

## Referential integrity

Repository validation rejects:

- duplicate canonical identities;
- broken local `uc-rust:` targets;
- unknown external namespace authorities;
- self-relations and duplicate edges;
- forbidden inverse relation codes;
- dependency and supersession cycles;
- isolated normative records where the family requires architecture traceability.

Unavailable external authorities remain explicit findings and are never replaced by locally invented copies.

## Diagnostics contract

Every blocking diagnostic identifies:

```text
file:field: [rule] explanation
```

Rule identifiers are stable enough for fixtures and CI assertions. Validation runs without Yukh or external infrastructure.

## Commands

```bash
python scripts/validate_records.py docs/knowledge/records
python scripts/validate_record_graph.py docs/knowledge/records
python scripts/validate_record_changes.py --base-ref origin/main docs/knowledge/records
python tests/test_record_validator.py
```

## Completion invariants

Issue #59 is complete when canonical shape, identifiers, graph integrity and historical transitions are deterministic in CI; schema migrations are governed; and no external projection is required to read or validate the repository.