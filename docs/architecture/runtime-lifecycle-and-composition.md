# Runtime Lifecycle, Configuration and Explicit Composition

- Status: Design baseline
- Governing decision: ADR-0025
- Governing issue: #48
- Related: ADR-0021, ADR-0024, RFC-0002

## 1. Purpose

Define deterministic startup, readiness, degradation, quiesce, drain, shutdown, configuration and dependency composition for every UC Rust runtime profile.

## 2. Lifecycle state machine

| State | Accepts new work | Required behavior |
|---|---:|---|
| Created | No | Record process/runtime identity. |
| Bootstrapping | No | Load and validate configuration, manifests and compatibility. |
| Starting | No | Construct and start components in dependency order. |
| Ready | Yes | Admit the declared Operation set. |
| Degraded | Bounded subset | Admit only Operations whose required capabilities remain valid. |
| Quiescing | Restricted | Reject new non-essential work and freeze admission policy. |
| Draining | No new ordinary work | Complete, checkpoint, defer or reconcile accepted work. |
| Stopping | No | Stop components in reverse dependency order and flush evidence. |
| Stopped | No | Release owned resources. |
| Failed | No | Expose failure evidence; never claim readiness. |

Allowed transitions are explicit. Unexpected component failure while Ready causes either Degraded, Quiescing or Failed according to dependency criticality and declared capability impact.

## 3. Readiness model

Readiness is computed from the configured profile, required Operation set, capability bindings, local data/manifests and dependency criticality.

Three signals are independent:

- **process liveness**: the process can make progress;
- **runtime readiness**: the configured mandatory Operation set is admissible;
- **capability operability**: a named Operation/capability is available, degraded or blocked.

An edge node can remain operational during WAN loss when its locally approved set remains valid.

## 4. Startup protocol

1. Establish immutable runtime identity.
2. Load configuration sources into a candidate snapshot.
3. Validate schema, signatures, secrets references and compatibility envelope.
4. Resolve profile and composition plan.
5. Validate Operation manifests and Capability Realization bindings.
6. Construct components without publishing readiness.
7. Start components in topological dependency order.
8. Run startup probes and local recovery/reconciliation checks.
9. Atomically publish active configuration/composition revisions.
10. Enter Ready or a permitted Degraded state.

If a mandatory component fails, already-started components are stopped in reverse order. The runtime enters Failed and emits a startup failure bundle.

## 5. Configuration contract

A configuration snapshot contains:

```text
schema_version
configuration_revision
content_digest
runtime_identity
runtime_profile
source_revisions
operation_set
capability_bindings
component_settings
reload_classification
created_at
```

Precedence is deterministic and recorded. Recommended source order, from lowest to highest precedence:

1. compiled defaults;
2. profile defaults;
3. signed deployment configuration;
4. site/store configuration;
5. secret references resolved through an approved provider;
6. bounded emergency override with owner and expiry.

Unknown fields are rejected for normative configuration unless the schema explicitly allows an extension namespace.

## 6. Reload semantics

Reload uses a transactional protocol:

```text
observe candidate
 -> parse
 -> validate schema
 -> validate compatibility/security
 -> build change plan
 -> prepare affected components
 -> optional quiesce/drain
 -> atomic commit of new snapshot
 -> activate
 -> verify
```

On any pre-commit failure, the old snapshot remains active. On post-commit activation failure, the runtime follows the declared rollback or restart policy; silent partial adoption is forbidden.

Field classes:

| Class | Rule |
|---|---|
| dynamic_safe | New invocations see the new immutable snapshot; in-flight work retains its captured revision. |
| drain_then_reload | Affected Operations/components quiesce and drain before activation. |
| restart_required | Candidate is accepted for next process start but not applied in place. |
| immutable_identity | Change is rejected for the current runtime identity. |

## 7. Composition root

Composition is explicit code/configuration at the executable boundary. The root creates typed objects and passes typed ports to constructors. It may use generated registration data, but application code cannot perform arbitrary lookup.

Required outputs:

- composition manifest;
- active Operation set;
- selected Capability Realizations and policy revisions;
- dependency graph and criticality;
- redacted configuration digest;
- performance/economic scorecard reference.

## 8. Component lifecycle contract

Every lifecycle-managed component declares:

- component identity and version;
- dependencies;
- criticality and affected Operations;
- start/stop deadlines;
- readiness and health contribution;
- reload class and preparation/activation hooks;
- resource ownership;
- checkpoint, lease and recovery behavior;
- observability and economic attribution.

Start and stop calls are idempotent. A component must not publish readiness before its owned resources and recovery obligations are valid.

## 9. Quiesce and drain

Admission classes:

- `read_only_safe`;
- `mutating_drainable`;
- `critical_completion`;
- `background_checkpointable`;
- `must_reject_on_quiesce`.

During quiesce, accepted work retains its original deadline, context and configuration revision. New work is rejected using canonical dispositions, not transport-specific failures.

Drain completion requires one of: completed, durably deferred, safely cancelled, checkpointed, or explicitly marked indeterminate with reconciliation evidence.

## 10. Central and edge profiles

Central and edge use the same lifecycle protocol but different mandatory sets.

- Central may require global integrations and control-plane connectivity.
- Store-edge requires local persistence, approved offline manifests and the declared local Operation subset.
- Loss of WAN does not automatically make store-edge unready.
- Expired or incompatible local manifests can block only the affected Operations unless they invalidate runtime identity/security.

## 11. Failure cases

| Failure | Required disposition |
|---|---|
| Invalid configuration schema | Startup/reload rejected; previous active snapshot retained. |
| Missing mandatory secret | Failed startup or affected Operation blocked. |
| Optional provider unavailable | Degraded with explicit capability impact. |
| Mandatory local store unavailable | Not Ready/Failed. |
| WAN unavailable at edge | Continue approved offline subset; block/degrade others. |
| Reload activation fails | Roll back or enter controlled restart/quiesce; never partial silent state. |
| Shutdown deadline exceeded | Forced termination evidence and next-start recovery required. |
| Dependency cycle | Composition validation failure before start. |
| Capability binding incompatible | Affected Operation rejected before readiness publication. |

## 12. Architecture enforcement

Executable checks must prevent:

- mutable global singletons;
- environment/config reads from domain or application Operation code;
- service-locator access from Operations;
- delivery adapter ownership of provider selection;
- duplicate central/edge business implementations;
- component startup outside the lifecycle supervisor;
- unregistered background tasks surviving drain/shutdown.

## 13. Scorecard

Every profile composition reports at least:

- cold and warm startup time;
- readiness time by dependency;
- steady-state RSS and heap allocation;
- idle CPU;
- per-invocation lifecycle/config overhead;
- component and connection counts;
- failure isolation and restart blast radius;
- configuration rollout frequency and failure rate;
- infrastructure and provider cost per store/day and tenant/month.

## 14. Acceptance boundary

This document defines architecture. M1 must provide executable state-machine, reload, composition, edge-offline, drain/shutdown and scorecard evidence before gate #54 can claim implementation readiness.