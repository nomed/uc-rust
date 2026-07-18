# Architecture Record Model

- Status: Draft
- Governing issue: to be assigned

## Common record envelope

Every architecture record contains:

- `id`: stable project-wide identifier;
- `type`: controlled record type;
- `title`: concise human-readable name;
- `status`: lifecycle state;
- `version`: semantic content version;
- `owners`: accountable maintainers;
- `created_at` and `updated_at`;
- `provenance`: source session, issue, UC-BoK reference or external evidence;
- `scope`: bounded concern owned by the record;
- `summary`: normative intent;
- `relations`: typed edges to other records and evidence;
- `review`: reviewers, disposition and date;
- `evidence`: implementation and verification references;
- `supersedes` / `superseded_by` where applicable.

## Record families

### ADR — Architecture Decision Record

Captures a consequential decision, alternatives and consequences.

### RFC — Request for Comments

Develops a substantial proposal before one or more decisions are accepted.

### CR — Capability Record

Defines a business or platform capability independently from its implementation. Required fields include purpose, owner, actors, operations, events, state, authority, offline class, consistency, security, economics, runtime profiles, dependencies, extension points and acceptance evidence.

### RRR — Runtime Responsibility Record

Defines one runtime responsibility, its boundaries, lifecycle, dependencies, failure semantics, resource budgets and supported profiles. It must not duplicate capability business semantics.

### QAR — Quality Attribute Record

Defines one measurable quality concern, scenarios, stimulus, environment, response, measure, budget and evidence.

### ER — Economic Record

Defines an economic unit, cost drivers, allocation rules, baseline, thresholds and measurement evidence.

### SR — Security Record

Defines threats, trust boundaries, assets, controls, residual risk and verification.

### IR — Interface Record

Defines a public or internal contract, semantic owner, compatibility policy, schemas and conformance evidence.

### DR — Data Record

Defines a governed data concept, authority, lifecycle, classification, retention, replication and migration rules.

### TR — Technology Record

Records technology evaluation or selection when the decision needs evidence beyond an ADR summary.

### RR — Release Record

Defines release scope, included records, compatibility envelope, entry/exit gates and evidence bundle.

## Record versus evidence

A record is normative knowledge. Code, tests, benchmark results, traces, reports and deployment artifacts are evidence linked to records. Evidence does not silently change the record state.

## Extensibility rule

A new record type requires:

1. a distinct semantic owner;
2. a lifecycle or required fields not safely expressible through an existing type;
3. at least two expected concrete uses;
4. an ADR accepting the taxonomy extension.