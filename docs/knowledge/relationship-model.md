# Typed Relationship Model

- Status: Draft
- Governing issue: to be assigned

## Purpose

Relationships are semantic edges, not generic hyperlinks. Each edge has a controlled type, direction, source, target, rationale and optional validity interval.

## Core relation vocabulary

### Governance and proposal

- `proposes`: RFC -> record or change
- `decides`: ADR -> governed subject
- `supersedes`: record -> record
- `amends`: record -> record
- `waives`: decision -> constraint/gate

### Architecture and capability

- `defines`: record -> concept or contract
- `requires`: record -> record/capability/runtime
- `constrains`: QAR/SR/ADR -> subject
- `depends_on`: subject -> prerequisite
- `conflicts_with`: record -> record
- `extends`: capability/interface -> base subject

### Runtime and implementation

- `realized_by`: capability/runtime/interface -> implementation artifact
- `invoked_by`: Operation -> adapter/runtime path
- `deployed_in`: component -> runtime profile
- `persists`: operation/capability -> data record
- `publishes` / `consumes`: subject -> event/interface record

### Evidence and verification

- `implemented_by`: record -> code/PR/release evidence
- `verified_by`: record -> test/benchmark/operational evidence
- `measured_by`: ER/QAR/capability -> measurement evidence
- `violated_by`: constraint -> evidence/incident
- `derived_from`: record -> source evidence or UC-BoK concept

### Planning and release

- `included_in`: record/capability -> release record
- `blocks`: subject -> subject
- `delivers`: issue/release -> record/capability
- `traces_to`: project record -> UC-BoK/EbD/external record

## Relation constraints

- Relations are directed even when a reverse view can be inferred.
- `supersedes` must not form cycles.
- `depends_on` must be acyclic within a release critical path unless explicitly classified as mutual design coupling.
- `verified_by` targets immutable or versioned evidence.
- A generic `related_to` edge is forbidden in normative records; the author must select or propose a semantic relation.
- Cross-repository targets use globally resolvable identifiers.

## Edge metadata

Each relation may include:

- rationale;
- provenance;
- confidence;
- valid-from / valid-until;
- release scope;
- conformance status;
- review status.

## Graph views

The same model must support at least:

- capability-to-operation view;
- decision impact view;
- runtime dependency view;
- release scope and critical path;
- UC-BoK traceability;
- economic attribution;
- verification coverage;
- stale or orphaned record detection.