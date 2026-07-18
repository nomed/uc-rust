# Extension Registration Test Plan

- Governing issue: #52
- Governing decision: ADR-0029

## Objective

Prove that UC Rust can register governed Operations, adapters and provider integrations without hidden business hooks, unrestricted runtime lookup, direct persistence access or a stable dynamic binary ABI.

## Manifest and compatibility

- Accept a valid compile-time extension manifest.
- Accept runtime activation only for components already present in the approved deployment.
- Reject incompatible runtime, profile, Operation or adapter versions before activation.
- Reject duplicate Operation IDs, adapter bindings, permission actions and conflicting realization registrations.
- Reject unknown registration modes and undeclared dependencies.

## Dependency boundaries

Automated architecture tests must prevent extensions and extension adapters from importing:

- concrete repositories;
- internal transaction managers;
- domain event publishers;
- unrestricted service locators or registries;
- concrete providers outside their isolated provider adapter;
- provider SDK types in canonical contracts.

## Registration modes

- Prove explicit compile-time registration through a profile composition root.
- Prove enable/disable through a runtime manifest without loading arbitrary code.
- Prove a process-isolated extension through a versioned adapter protocol.
- Confirm no 1.0 scenario requires a stable shared-library ABI.

## Permissions and secrets

- Deny undeclared permissions.
- Verify least-privilege scopes by tenant, provider and capability.
- Verify secrets are obtained only through typed scoped handles.
- Verify activation, permission change and credential-scope change produce attributable audit evidence.

## Capability Realization integration

- Register a provider adapter and realization manifest through an extension package.
- Confirm registration only makes the realization eligible.
- Confirm the Capability Binder performs selection.
- Confirm REST/gRPC/CLI/worker adapters cannot call the registered provider directly.
- Confirm canonical Operation fixtures remain unchanged when the provider extension is enabled, disabled or replaced.

## Lifecycle and failure isolation

- Start components in dependency order and stop in reverse order.
- Optional extension failure degrades only declared Operations.
- Mandatory extension failure blocks readiness for the affected profile.
- Health freshness expiry becomes unknown rather than healthy.
- Quiesce/drain occurs before activation when required.
- Failed activation leaves the previous accepted composition active or the extension disabled.
- Partial mixed extension revisions are impossible.

## Upgrade and rollback

- Prepare and validate a candidate revision before activation.
- Preserve active revision for in-flight invocations.
- Roll back after failed post-activation verification.
- Reject rollback when configuration/state migration is not backward compatible and no safe strategy exists.
- Record immutable activation and rollback history.

## Resource and economic budgets

Measure startup latency, steady memory, CPU, concurrency, network, storage, telemetry volume and provider charges. Attribute extension framework overhead separately from adapter, Operation and realization cost. Budget breach must be visible gate evidence.

## Security and privacy

- Reject generic all-internal-access permissions.
- Prevent cross-tenant configuration or secret access.
- Prevent raw business payload logging by default.
- Verify protected diagnostics do not cross public adapter contracts.
- Verify security, fiscal, legal and authority blockers cannot be bypassed by extension availability policy.

## M1 evidence bundle

The bundle includes manifests, schema validation output, dependency graph checks, activation/rollback logs, permission audit evidence, failure-isolation tests, provider-binding proof and resource/cost scorecards.