# Controlled Architecture Record Taxonomy

- Status: Draft
- Governing issue: #56
- Governing decision: ADR-0022

## Taxonomy principles

A record family exists only when it has a distinct semantic responsibility. Differences in audience, layout or file location do not justify a new family.

Records are normative. Sessions, handoffs, notes, diagrams, code and test reports may support records but are not automatically record families.

## Accepted baseline candidates

### ADR — Architecture Decision Record

**Owns:** a consequential decision, alternatives, rationale and consequences.

**Does not own:** the complete design of a capability or runtime responsibility.

### RFC — Request for Comments

**Owns:** development and review of a substantial proposal before final disposition.

**Does not own:** an accepted decision after it has been split or dispositioned into normative records.

### CR — Capability Record

**Owns:** the implementation-independent definition of one business or platform capability.

**Required concerns:** purpose, actors, operations, events, state, authority, offline behavior, consistency, security, economics, quality budgets, runtime profiles, dependencies, extension points and acceptance evidence.

### RRR — Runtime Responsibility Record

**Owns:** one reusable runtime responsibility and its operational contract.

**Required concerns:** boundary, lifecycle, invocation or execution semantics, dependencies, failure behavior, resource budget, supported profiles, observability and compatibility.

**Does not own:** business semantics belonging to a Capability Record.

### QAR — Quality Attribute Record

**Owns:** one measurable quality attribute expressed through scenarios and budgets.

**Required concerns:** source, stimulus, environment, artifact, response, response measure, threshold and evidence.

### ER — Economic Record

**Owns:** one economic unit or governed cost/revenue model.

**Required concerns:** unit, cost drivers, allocation, baseline, thresholds, measurement environment and evidence freshness.

### SR — Security Record

**Owns:** one bounded security concern, threat model or control set.

**Required concerns:** assets, actors, trust boundaries, threats, controls, residual risk, exceptions and verification.

### IR — Interface Record

**Owns:** the semantic contract and compatibility policy of an interface.

**Required concerns:** owner, consumers, operations/messages, schema, error semantics, versioning, compatibility and conformance evidence.

### DR — Data Record

**Owns:** one governed data concept or dataset.

**Required concerns:** semantic definition, authority, classification, lifecycle, retention, replication, consistency, lineage and migration.

### TR — Technology Record

**Owns:** structured evaluation or governed profile of a technology where evidence is too substantial for an ADR alone.

**Constraint:** selection remains an ADR. A TR supplies evaluation evidence and operating constraints.

### RR — Release Record

**Owns:** one release's governed composition and evidence bundle.

**Required concerns:** included records, excluded/deferred scope, compatibility envelope, migration, entry and exit gates, known exceptions and release evidence.

## Non-record artifacts

The following remain supporting artifacts unless a future ADR decides otherwise:

- session notes;
- handoffs;
- decision logs;
- diagrams;
- glossaries;
- implementation plans;
- test reports;
- benchmark reports;
- runbooks;
- incidents;
- code modules and crates.

They are referenced through provenance or evidence and may have their own operational templates.

## Potential consolidation decisions

The baseline intentionally avoids additional families for:

- `Operation Record`: operations belong to a CR and may be realized through an RRR invocation contract;
- `Event Record`: event semantics belong to an IR, with ownership from a CR;
- `Policy Record`: policy is represented by the owning CR, SR, DR, QAR or ADR;
- `Deployment Record`: deployment evidence and runtime profiles belong to RRR/RR;
- `Agent Record`: agent responsibilities are modeled as capabilities/runtime responsibilities unless a distinct semantic need is demonstrated.

## Extension test

Before proposing a new family, answer:

1. Which normative question cannot be answered by an existing family?
2. Which existing family would otherwise have conflicting ownership?
3. What required fields and integrity rules are unique?
4. Which two concrete records will exist within the 1.0 horizon?
5. How will existing tools and schemas migrate?

Failure to answer any item means the concern should remain a subtype, section, relation or supporting artifact rather than a new record family.
