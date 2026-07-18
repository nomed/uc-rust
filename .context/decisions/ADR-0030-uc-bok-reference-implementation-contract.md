# ADR-0030 — UC-BoK Reference Implementation and Traceability Contract

- Status: Reviewable for acceptance
- Date: 2026-07-19
- Governing issue: #38
- Related specification issue: nomed/uc-bok#9
- Normative peer contract: nomed/uc-bok/specifications/018-uc-rust-integration/operating-contract.md
- Related: ADR-0021, ADR-0024, ADR-0025, ADR-0026, ADR-0028, ADR-0029

## Context

UC-BoK is the normative Unified Commerce knowledge and specification repository. UC Rust is its reference implementation and executable feedback environment. The relationship must be explicit, versioned, bidirectional and auditable; neither repository may silently assume conformance or divergence.

UC-BoK already publishes accepted operating contract 1.0 under specification `018-uc-rust-integration` and a generated stable-identifier manifest. UC Rust needs the reciprocal implementation contract and local mapping rules.

## Decision

UC Rust adopts the UC-BoK ↔ UC Rust cross-repository operating contract version 1.0 and mirrors its obligations locally.

```text
UC-BoK normative release/revision
  -> stable identifiers and traceability manifest
    -> UC Rust impact assessment
      -> implementation mappings and executable evidence
        -> drift/coverage disposition
          -> reciprocal feedback where required
```

UC Rust SHALL record the exact UC-BoK source revision or release used by every traceability baseline. A claim of conformance is valid only for the recorded revision and mapped identifiers.

## Authority boundary

- UC-BoK defines normative concepts, principles, capabilities, business objects, relationships, constraints and schemas.
- UC Rust implements canonical domain/application contracts and produces executable evidence.
- UC Rust may challenge UC-BoK through structured evidence but cannot silently redefine normative identifiers.
- UC-BoK changes do not enter UC Rust silently; each material change requires impact assessment and disposition.
- Intentional divergence requires attributable accepted decisions in both repositories.

## Identifier rule

Every governed mapping uses the pair `(ucbok_type, ucbok_id)`, not `ucbok_id` alone, because the same textual identifier may exist in different namespaces.

Supported normative types initially include:

- `capability`;
- `business-object`;
- `event`;
- `process`;
- `application`;
- `lifecycle`;
- `pattern`;
- other types explicitly published by the normative manifest.

UC Rust artifacts may reference stable identifiers only when they resolve against the recorded UC-BoK manifest revision.

## Mapping statuses

UC Rust uses these implementation dispositions:

- `implemented` — verified implementation and evidence exist;
- `partial` — bounded subset implemented; missing scope is explicit;
- `planned` — accepted roadmap ownership exists but implementation evidence does not;
- `not-applicable` — normative element is outside the declared UC Rust product/reference scope, with rationale;
- `intentionally-divergent` — accepted reciprocal decision documents the incompatibility or deliberate deviation.

Absence from the local catalog means `unmapped`, never `implemented`.

## Evidence contract

Each mapping declares:

- UC-BoK type and stable identifier;
- normative source revision;
- UC Rust status;
- owning capability/context;
- implementation artifacts such as crates, Operations, public contracts and migrations;
- verification artifacts such as tests, fixtures, architecture evidence or gate references;
- compatibility and migration notes;
- reciprocal decision or issue links when divergent or blocked.

A documentation link alone cannot prove `implemented`.

## Cross-repository issue contract

Every reciprocal issue includes:

- source repository and issue/PR/commit;
- target repository;
- stable fingerprint;
- affected `(type, id)` pairs;
- affected UC Rust artifacts where applicable;
- change type;
- normative and implementation impact;
- reproducible evidence;
- expected resolution and acceptance criteria;
- compatibility/migration impact;
- reciprocal issue link.

Fingerprint input is canonicalized from source repository, source reference and sorted typed identifiers. Existing open issues with the same fingerprint receive additional evidence instead of duplicates.

## Lifecycle rules

### UC-BoK to UC Rust

A normative release/change produces an adoption impact assessment. UC Rust records one of:

- `adopted`;
- `partial-adoption`;
- `intentional-divergence`.

### UC Rust to UC-BoK

Ambiguity, contradiction, missing concept or impractical normative constraint produces implementation feedback. The UC Rust issue cannot be considered resolved until UC-BoK records an attributable disposition:

- `accepted`;
- `accepted-with-modification`;
- `rejected`;
- `deferred`.

## Drift and coverage

Drift reporting separates:

- normative inventory total;
- mapped versus unmapped inventory;
- status counts by type and domain;
- stale normative revisions;
- broken artifact/evidence references;
- intentional divergences lacking reciprocal acceptance;
- implemented claims without executable evidence.

Coverage percentage alone is not a release gate. P0 capability and contract mappings may block release even when aggregate coverage is high.

## CI obligations

M1 and later gates must validate:

1. every typed identifier resolves against the pinned normative manifest;
2. statuses are from the accepted vocabulary;
3. required fields exist for each status;
4. mapped repository paths exist;
5. `implemented` mappings include qualifying evidence;
6. duplicate mapping keys and fingerprints are rejected;
7. intentional divergences include reciprocal accepted decisions;
8. the pinned UC-BoK revision is available and has not been silently changed.

These are required proofs; this ADR does not falsely claim that all automation already exists.

## Consequences

- UC Rust can state exactly which UC-BoK revision and elements it implements.
- Specification drift becomes visible and attributable.
- Implementation evidence can improve the normative model without collapsing specification authority into code.
- Provisional roadmap labels cannot masquerade as stable UC-BoK identifiers.
- Cross-repository automation remains replaceable because the contract is data- and workflow-oriented rather than tool-specific.

## Rejected alternatives

- untyped identifier strings;
- repository-wide binary conformance claims;
- treating absent mappings as not-applicable;
- silently adopting the latest UC-BoK main branch;
- closing implementation feedback without normative disposition;
- using documentation presence as implementation proof;
- allowing intentional divergence in only one repository.

## Design evidence

- `docs/governance/uc-bok-reference-implementation-contract.md`;
- `governance/traceability/uc-bok-mappings.yaml`;
- `docs/testing/uc-bok-traceability-test-plan.md`;
- normative manifest published by `nomed/uc-bok`.

## Implementation evidence required by M1

- pinned normative revision and imported/validated identifier inventory;
- schema and CI validation for the local mapping catalog;
- generated drift and coverage report;
- at least one end-to-end mapping from UC-BoK identifier to Operation/contract/test evidence;
- reciprocal issue creation and deduplication proof;
- stale revision and broken reference rejection tests;
- Project Ready and quality-model integration.

This ADR can be accepted before all executable automation exists; release gates must not claim those proofs until produced.