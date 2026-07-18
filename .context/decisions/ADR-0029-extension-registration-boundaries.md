# ADR-0029 — Extension, Plugin and Capability Registration Boundaries

- Status: Reviewable for acceptance
- Date: 2026-07-19
- Governing issue: #52
- Parent epic: #46
- Related: ADR-0021, ADR-0024, ADR-0025, ADR-0028, RFC-0002

## Context

UC Rust must allow new capabilities, adapters and provider integrations without exposing internal persistence, hidden hooks or transport-specific business services. Extension packaging must remain distinct from governed Capability Realization: an external service may realize a canonical capability without being a binary plugin, while an extension package may register adapters, Operations or providers without acquiring unrestricted runtime authority.

## Decision

UC Rust 1.0 adopts **manifest-governed registration with compile-time composition as the default**. Runtime registration may activate pre-built, trusted components described by accepted manifests. Arbitrary dynamic native-library loading and a stable binary plugin ABI are deferred beyond 1.0.

```text
Extension package
  -> signed/approved manifest
    -> compatibility and policy validation
      -> explicit composition root registration
        -> typed runtime ports
```

An extension may declare:

- canonical Operations and descriptors;
- delivery-adapter bindings;
- Capability Realization/provider adapters;
- permissions and policy actions;
- typed configuration schemas;
- lifecycle-managed components;
- health, observability and economic evidence contracts;
- fixtures, compatibility ranges and rollback metadata.

Registration never grants direct repository, transaction-manager, secret-store, provider-registry or service-locator access.

## Registration modes

- `compile_time`: component linked into the executable and registered by the profile composition root;
- `runtime_manifest`: pre-built trusted component enabled or disabled through accepted configuration and manifest validation;
- `process_isolated`: external process connected through a versioned protocol and governed adapter;
- `dynamic_binary`: deferred beyond 1.0 and requires a future accepted ABI/security decision.

Runtime manifest activation is not arbitrary code loading. Only artifacts present in the approved deployment composition may be activated.

## Compatibility

Every extension manifest declares:

- extension ID and semantic version;
- publisher/owner and trust classification;
- supported UC Rust/runtime/profile ranges;
- provided and required contracts;
- Operation, adapter and realization version ranges;
- configuration schema and migration rules;
- permissions and authority requirements;
- lifecycle, health and resource budgets;
- evidence, fixtures and conformance results;
- rollback and disable semantics.

Compatibility is checked before activation. Ambiguous registration, duplicate authority or conflicting bindings fail closed.

## Capability Realization relationship

ADR-0024 governs how a canonical capability is realized. ADR-0029 governs how packaged components become eligible for composition.

- registration does not select a realization;
- the Capability Binder selects only among accepted and eligible realization manifests;
- an external delegated service may be represented by a provider adapter registered through an extension package;
- consumer-facing adapters never call extension providers directly;
- extension identity never replaces canonical Operation or capability identity.

## Security and authority

Extensions operate with least privilege. Permissions are explicit, versioned and auditable. Secrets are obtained only through typed, scoped credential ports. Extensions cannot introspect unrelated configuration, tenants, repositories or providers.

Activation, upgrade, disable, rollback and permission changes require attributable audit evidence. Security, fiscal, legal and authority constraints take precedence over availability or cost.

## Failure isolation and rollback

Each extension declares criticality, start/stop deadlines, dependency graph, health contribution and failure domain. Optional extensions degrade only declared Operations. Mandatory extension failure blocks readiness for the affected profile or Operation set.

Upgrade follows prepare, validate, quiesce/drain where required, atomic activation and verification. Failed activation restores the prior accepted composition or leaves the extension disabled; partial mixed revisions are forbidden.

## Resource and economic budgets

Extensions declare startup time, memory, CPU, concurrency, network, storage, telemetry and provider-cost budgets. Budget evidence is attributed separately from canonical Operation and realization cost. Economic preference cannot override semantic, authority, security or quality eligibility.

## Architecture enforcement

Forbidden dependencies include:

```text
extension adapter -> repository
extension adapter -> concrete provider selected directly
extension -> unrestricted service locator
extension -> internal transaction manager
extension -> hidden business hook
extension contract -> provider SDK type
```

Permitted dependencies are canonical contracts, typed runtime ports and explicitly granted capability/provider interfaces.

## Consequences

- UC Rust 1.0 gains governed extensibility without committing to a fragile native ABI.
- Compile-time safety remains the default.
- External services, adapters and future capabilities compose through the same governance model.
- Provider binding remains separate from packaging and registration.
- Extensions can be disabled or rolled back without rewriting canonical business semantics.

## Rejected alternatives

- unrestricted service locator;
- arbitrary shared-library loading in 1.0;
- extension-specific repository access;
- provider-specific consumer Operations;
- registration that implicitly selects providers;
- hidden callback hooks into domain internals;
- one global plugin permission;
- best-effort upgrade with mixed revisions.

## Design evidence

- `docs/architecture/extension-registration-boundaries.md`;
- `governance/schemas/extension-manifest.schema.json`;
- `docs/testing/extension-registration-test-plan.md`.

## Implementation evidence required by M1

- compile-time and runtime-manifest registration proof;
- compatibility rejection and duplicate-registration tests;
- permission and dependency-boundary enforcement;
- activation, disable and rollback proof;
- failure isolation and readiness impact tests;
- provider adapter registered without adapter-to-provider bypass;
- resource and cost scorecards;
- proof that no stable dynamic binary ABI is required for 1.0.

This ADR can be accepted before executable M1 evidence exists; gate #54 must not claim those proofs until produced.