# Operation Architecture Enforcement

- Status: Design baseline
- Governing issue: #47
- Governing decision: ADR-0021

## Purpose

Define executable dependency and source-boundary checks that prevent delivery adapters, workers and schedulers from creating private business-service paths around canonical Operations.

## Intended crate/layer roles

```text
uc-domain          pure domain types, rules and invariants
uc-application     typed Operations and capability-oriented ports
uc-operation       invocation contracts, descriptors and canonical outcomes
uc-runtime         composition, invocation pipeline and binding
uc-adapter-*       REST, gRPC, CLI, worker, scheduler and synchronization adapters
uc-provider-*      provider-specific capability adapters
uc-persistence-*   repository and unit-of-work implementations
```

Final crate names may be consolidated, but these logical dependency roles remain enforceable.

## Allowed dependency direction

```text
adapter -> operation/runtime/application contract
runtime -> operation/application ports
application -> domain + operation contract + capability ports
domain -> standard/core domain dependencies only
provider/persistence -> application ports + domain contract
```

Composition roots may depend on concrete adapters/providers to assemble the runtime. Business code may not depend on the composition root.

## Forbidden boundaries

Automated checks must fail when:

1. a delivery adapter imports a repository, unit-of-work implementation or persistence client;
2. a delivery adapter imports a provider SDK or provider-specific adapter;
3. a worker/scheduler defines a business contract not represented by a canonical Operation;
4. an Operation imports transport frameworks, request/response types or broker-specific message types;
5. domain/application code imports concrete provider or persistence implementations;
6. an Operation accesses a global registry, unrestricted service locator or mutable global state;
7. an adapter publishes domain/integration events outside the Operation outcome/effect contract;
8. provider payloads appear in public Operation input, output or error types;
9. central and edge profiles contain distinct implementations of the same business Operation;
10. telemetry code serializes canonical input/output payloads by default.

## Enforcement mechanisms

M1 must implement a combination of:

- Cargo dependency graph assertions;
- crate-private/public API boundaries;
- compile-fail tests for forbidden imports where practical;
- source-tree policy tests for framework/provider type leakage;
- manifest-to-registration consistency tests;
- fixture coverage proving all adapters resolve an existing Operation ID;
- lints denying direct provider/repository dependencies from adapter crates;
- CI checks that fail on an unregistered adapter business handler.

The enforcement must inspect semantic dependency ownership, not rely only on directory naming.

## Adapter registration rule

Every adapter route, command, worker message or scheduled job that causes business behavior must declare the canonical Operation ID it invokes. CI compares registrations against validated Operation manifests.

An adapter may have transport-only health, discovery or static-content handlers. Such exceptions must be explicitly classified and cannot mutate business state.

## Provider isolation rule

Provider-specific code is visible only through capability-oriented ports and approved Capability Realization registrations. Architecture tests verify:

- provider SDK types do not cross the adapter boundary;
- delivery adapters cannot reference realization IDs;
- the Capability Binder is used only by runtime composition/invocation infrastructure;
- Operations cannot enumerate arbitrary providers.

## Central/edge equivalence rule

Central and store-edge composition may bind different persistence or capability realizations, but must register the same canonical Operation implementation for the same semantic version. CI compares profile manifests and rejects duplicate profile-specific business implementations.

## Evidence output

CI must produce an architecture evidence report containing:

- dependency violations;
- adapter-to-Operation registration map;
- Operation-to-capability-port map;
- profile-to-Operation implementation map;
- provider SDK leakage findings;
- unregistered business handlers;
- accepted exceptions with owner and expiry.

## Acceptance boundary

This document completes the design of architecture enforcement. #47 and ADR-0021 may be accepted as architecture decisions before Rust crates exist. Gate #54 and M1 must not claim executable enforcement until these checks run against implementation code.