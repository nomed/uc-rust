# Architecture Record Model

- Status: Draft
- Governing issue: #56
- Governing decision: ADR-0022
- Related: `common-record-envelope.md`, `record-taxonomy.md`, `record-lifecycle.md`, `relationship-model.md`

## Purpose

The Architecture Record Model is the project-wide language for expressing normative architecture knowledge. It separates governed intent from implementation and evidence, while allowing UC-BoK, UC Rust, Yukh and Economics by Design to share stable identities and typed relationships.

## Model layers

```text
Common envelope
  + record-family body
  + typed relations
  + lifecycle and review metadata
  + evidence references
```

The common envelope and record-family body are normative. Rendered pages, project fields and graph views are projections.

## Common envelope

Every architecture record contains:

- `id`: stable namespaced identifier;
- `type`: controlled record family;
- `schema_version`: representation and validation version;
- `content_version`: governed semantic version;
- `title`: concise human-readable name;
- `summary`: normative intent;
- `status`: lifecycle state;
- `owners`: accountable maintainers;
- `created_at` and `updated_at`;
- `scope` and, where needed, `non_goals`;
- `provenance`: typed origins;
- `relations`: typed directional edges;
- `review`: reviewer requirements, disposition and freshness;
- `evidence`: implementation and verification references;
- deprecation, waiver and supersession metadata where applicable.

Detailed semantics are defined in `common-record-envelope.md`.

## Record families

### ADR — Architecture Decision Record

Captures a consequential decision, alternatives, rationale and consequences.

### RFC — Request for Comments

Develops a substantial proposal before one or more decisions or records are accepted.

### CR — Capability Record

Defines a business or platform capability independently from implementation. It owns purpose, actors, operations, events, state, authority, offline class, consistency, security, economics, runtime profiles, dependencies, extension points and acceptance evidence.

### RRR — Runtime Responsibility Record

Defines one runtime responsibility, its boundaries, lifecycle, dependencies, failure semantics, resource budgets, compatibility and supported profiles. It must not duplicate capability business semantics.

### QAR — Quality Attribute Record

Defines one measurable quality concern through scenarios, stimulus, environment, response, measure, threshold and evidence.

### ER — Economic Record

Defines an economic unit, cost drivers, allocation rules, baseline, thresholds and measurement evidence.

### SR — Security Record

Defines a bounded threat/control concern, trust boundaries, assets, controls, residual risk and verification.

### IR — Interface Record

Defines an interface's semantic owner, contract, error behavior, compatibility policy, schemas and conformance evidence.

### DR — Data Record

Defines a governed data concept, authority, lifecycle, classification, retention, lineage, replication and migration rules.

### TR — Technology Record

Provides structured technology evaluation and operating constraints. A technology selection remains an ADR.

### RR — Release Record

Defines release scope, included and deferred records, compatibility envelope, entry/exit gates, migration and evidence bundle.

The ownership boundaries and anti-proliferation rules are defined in `record-taxonomy.md`.

## Record versus evidence

A record is normative knowledge. Code, tests, benchmark results, traces, reports and deployment artifacts are evidence linked to records. Evidence does not silently change lifecycle status, conformance or release inclusion.

## Orthogonal dimensions

The model keeps separate:

- lifecycle status;
- epistemic confidence;
- implementation conformance;
- verification freshness;
- release inclusion.

A record can therefore be accepted but not implemented, implemented but not currently verified, or verified but excluded from a specific release.

## Extensibility rule

A new record type requires:

1. a distinct semantic owner;
2. required fields or integrity rules not safely expressible through an existing type;
3. at least two expected concrete uses within the planning horizon;
4. an ADR accepting the taxonomy extension;
5. schema and migration impact.
