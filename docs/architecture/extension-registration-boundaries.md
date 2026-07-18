# Extension Registration Boundaries

- Status: Design baseline
- Governing issue: #52
- Governing decision: ADR-0029

## Purpose

Define how UC Rust packages and registers Operations, adapters, provider integrations, permissions, configuration and lifecycle components without granting unrestricted access to application internals.

## Boundary model

```text
package artifact
  -> extension manifest
  -> trust and compatibility validation
  -> profile composition root
  -> typed registration ports
  -> lifecycle supervision
  -> canonical Operation invocation / Capability Binder
```

Packaging, registration and provider selection are separate concerns.

## Extension package contents

A package may include:

- Operation implementations and descriptors;
- REST, gRPC, CLI or worker bindings;
- provider adapters eligible as Capability Realizations;
- lifecycle-managed technical components;
- permission/action declarations;
- configuration schemas and defaults;
- health contributors and Evidence Envelope emitters;
- canonical fixtures and conformance evidence;
- migration, disable and rollback metadata.

The package cannot declare hidden callbacks into domain internals or private persistence schemas.

## Registration protocol

Registration is performed only by the composition root through typed registrars:

- `OperationRegistrar`;
- `AdapterBindingRegistrar`;
- `RealizationRegistrar`;
- `PermissionRegistrar`;
- `ConfigurationSchemaRegistrar`;
- `LifecycleComponentRegistrar`;
- `EvidenceContractRegistrar`.

Registrars validate identity uniqueness, compatibility ranges, dependency graph, authority and resource budgets. They return a bounded registration result and never expose a mutable global registry to Operations.

## Supported 1.0 modes

### Compile-time

The component is linked and explicitly registered by the executable profile. This is the preferred mode because type and dependency checks remain available during build and test.

### Runtime manifest activation

A component already present in the deployment artifact may be enabled or disabled using an accepted manifest/configuration revision. Activation does not fetch or load arbitrary machine code.

### Process-isolated extension

A separately deployed process communicates through a versioned protocol. It is treated as an adapter or delegated realization, with deadlines, idempotency, health and economic evidence governed by existing contracts.

### Dynamic binary loading

Native shared-library loading, ABI stability and in-process untrusted plugins are out of scope for 1.0.

## Validation sequence

1. verify manifest schema and stable identity;
2. verify publisher/trust evidence;
3. verify runtime and profile compatibility;
4. verify required/provided contract ranges;
5. verify permissions and authority boundaries;
6. verify dependency graph and lifecycle deadlines;
7. verify configuration migration and rollback path;
8. verify resource/economic budgets;
9. reject duplicate/conflicting registration;
10. atomically publish the accepted composition revision.

## Upgrade and rollback

Upgrades create a new extension revision. The runtime prepares and validates the candidate before changing active composition. Depending on declared reload class, activation is dynamic-safe, drain-then-reload or restart-required.

A failed verification restores the previous accepted revision or leaves the extension disabled. In-flight invocations retain the registration/configuration/binding revisions captured at admission.

## Failure isolation

Extensions declare:

- mandatory, profile-mandatory, optional or on-demand criticality;
- affected Operations/capabilities;
- startup and shutdown deadlines;
- concurrency and bulkhead limits;
- health freshness and readiness contribution;
- recovery and rollback behavior.

Failure may degrade only the declared scope. It cannot silently change provider authority, fiscal behavior, legal guarantees or offline promises.

## Permission model

Permissions are explicit actions over scoped resources. Registration cannot request a generic `all_internal_access` permission. Credential access is provider/tenant/scope-specific through typed secret handles. Every activation and permission change produces attributable audit evidence.

## Registration versus realization

An extension package can register a provider adapter and realization manifest. The Capability Binder still performs realization eligibility and selection for each canonical Operation. Delivery adapters and Operations never access the provider registry directly.

```text
extension registration -> eligible component
Capability Binder       -> selected realization
Operation               -> canonical capability port
```

## Resource and economics contract

The extension manifest declares budgets for startup latency, steady memory, CPU, queue/concurrency, network, storage, telemetry volume and direct provider charges. Measurements use ADR-0026 evidence and distinguish:

- extension framework overhead;
- adapter overhead;
- Operation cost;
- realization/provider cost.

## Enforcement

Architecture tests must reject imports from extension/adapters into concrete repositories, transaction managers, internal event publishers, concrete providers outside their adapter, or unrestricted registries. Canonical contracts must contain no extension-framework or provider SDK types.

## 1.0 proof

A proving package should register one adapter and one provider realization, be activatable/disableable by manifest, fail compatibility validation on an incompatible revision, roll back cleanly after failed activation, and preserve the canonical Operation contract throughout.