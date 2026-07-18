# ADR-0025 — Runtime Lifecycle, Configuration and Explicit Composition

- Status: Accepted
- Date: 2026-07-18
- Governing issue: #48
- Parent epic: #46
- Related: ADR-0021, ADR-0024, RFC-0002, #49, #50, #51, #52

## Context

UC Rust must run the same canonical Operations in central and store-edge profiles while tolerating partial dependency failure, staged configuration changes, WAN disruption and controlled shutdown. Hidden globals, implicit dependency discovery and configuration mutation would make runtime behavior non-deterministic and prevent reliable edge operation.

## Decision

UC Rust adopts an explicit lifecycle, immutable revisioned configuration snapshots and profile-specific composition roots.

A runtime instance moves through the governed states:

```text
Created -> Bootstrapping -> Starting -> Ready
                           |           |
                           v           v
                         Failed      Degraded
                                       |
Ready/Degraded -> Quiescing -> Draining -> Stopping -> Stopped
```

`Failed` and `Stopped` are terminal for that process instance. Recovery creates a new lifecycle attempt rather than mutating history.

## Lifecycle semantics

- **Created**: process exists; no external work accepted.
- **Bootstrapping**: configuration sources are loaded, parsed, validated and compatibility-checked.
- **Starting**: composition root constructs dependencies and starts components in dependency order.
- **Ready**: the declared mandatory Operation set is admissible for this profile.
- **Degraded**: the runtime remains useful, but one or more optional or bounded capabilities are unavailable or operating under an explicitly declared degraded policy.
- **Quiescing**: new non-essential work is rejected; admission is frozen according to Operation class.
- **Draining**: accepted work is allowed to complete within deadlines; background work checkpoints or relinquishes leases.
- **Stopping**: components stop in reverse dependency order and durable evidence is flushed.
- **Stopped**: no work is accepted and all owned resources are released.
- **Failed**: startup or lifecycle invariant failed and safe service cannot be claimed.

Liveness, readiness and operational capability are separate signals. A live process may be not ready; an edge runtime may be degraded but operational for an approved offline subset.

## Configuration

Configuration is typed, schema-versioned and loaded into immutable snapshots identified by a revision and content digest. Operations receive only the governed facts they require through invocation context or typed ports; they never read global configuration directly.

Every configuration field is classified as one of:

- `dynamic_safe`: can be atomically replaced for future invocations;
- `drain_then_reload`: requires quiesce/drain of affected work;
- `restart_required`: requires a new process composition;
- `immutable_identity`: cannot change for an existing runtime identity.

Reload is prepare/validate/commit. Partial mutation is forbidden. A rejected candidate leaves the active snapshot unchanged and emits auditable evidence.

## Composition

Each executable profile has one explicit composition root that:

1. selects the runtime profile;
2. loads an accepted configuration snapshot;
3. constructs concrete providers and adapters;
4. validates capability/Operation/realization compatibility;
5. wires typed ports into Operations;
6. registers lifecycle dependencies and health contributors;
7. publishes a redacted composition manifest and scorecard.

No unrestricted service locator, global mutable singleton or runtime reflection hook is available to application Operations.

Central and store-edge may use different concrete providers, but the same canonical Operation implementation and semantic version must be reused unless an accepted divergence exists.

## Dependency criticality

Components are declared as:

- `mandatory`: failure blocks readiness;
- `profile_mandatory`: required only for named profiles or Operation sets;
- `optional`: failure degrades declared capability only;
- `on_demand`: initialized lazily behind a typed port with bounded failure semantics.

A dependency failure cannot silently change authority, security, fiscal, legal or offline promises.

## Edge operation

Store-edge readiness is evaluated against its locally declared operational set, not against universal central connectivity. WAN-dependent realizations may be unavailable while approved local Operations remain Ready or Degraded. Configuration and capability manifests required for offline operation must be locally durable, revisioned and freshness-governed.

## Shutdown

Shutdown is deadline-bounded and idempotent. Admission stops before dependency teardown. Mutating work, outbox/effects, idempotency records and owned leases follow declared drain/checkpoint/reconcile rules. Forced termination after the deadline is recorded as an incomplete shutdown requiring recovery evidence on next start.

## Economics and performance

Composition choices require scorecards covering startup time, steady-state memory, idle CPU, per-invocation overhead, dependency count, failure isolation, operational complexity and cost-to-serve. Cheaper composition cannot weaken semantic, authority, security or reliability constraints.

## Rejected alternatives

- global mutable configuration;
- Operations reading environment variables or framework configuration directly;
- service locator or arbitrary dependency lookup;
- readiness equal to process liveness;
- central connectivity as a universal edge readiness requirement;
- in-place partial configuration mutation;
- shutdown by process kill without quiesce/drain semantics;
- provider selection performed by delivery adapters.

## Design evidence

- `docs/architecture/runtime-lifecycle-and-composition.md`;
- `governance/schemas/runtime-profile-config.schema.json`;
- `docs/testing/runtime-lifecycle-configuration-test-plan.md`;
- RFC-0002 integration.

## Implementation evidence required by M1

- executable lifecycle state-machine tests including invalid transitions;
- startup rollback and reverse-order shutdown proof;
- atomic configuration reload and rejected-candidate proof;
- central and store-edge composition manifests;
- offline edge readiness proof with central dependency unavailable;
- quiesce/drain behavior with in-flight mutating Operations and workers;
- architecture tests preventing globals and service-locator access;
- startup, memory, idle CPU and cost scorecards.

This ADR can be accepted as an architectural decision before executable M1 evidence exists. Gate #54 must not claim those implementation proofs until they are produced.