# ADR-0007: Replaceable infrastructure adapters

- Status: Accepted
- Date: 2026-07-18
- Decision owners: Daniele Favara
- Governing issue: #24

## Context

UC Rust must remain independent from specific database, cache and file/object storage technologies. PostgreSQL, SQLite, Redis, in-memory implementations, S3-compatible services and local filesystems have different APIs and operational properties. Allowing those SDKs or data types into the application core would couple business use cases to infrastructure and make replacement expensive.

## Decision

Use ports-and-adapters for all outbound infrastructure capabilities.

The domain and application layers define capability-oriented ports in business terms. Infrastructure crates implement those ports for specific providers.

Required categories include:

- repositories and transaction/unit-of-work boundaries;
- cache lookup, write, invalidation and expiry;
- blob/object storage read, write, delete, metadata and streaming;
- clocks, identifiers, messaging and other external capabilities as they arise.

Provider types, query builders, connection handles, SDK errors, bucket concepts, filesystem paths and serialization details must not cross into domain or application public contracts.

Provider selection occurs in the composition root through configuration and dependency injection.

Each port must state the guarantees required by its use cases, including where relevant:

- atomicity and transaction scope;
- optimistic or conditional writes;
- consistency expectations;
- ordering;
- TTL and invalidation semantics;
- maximum object sizes;
- streaming and range-read support;
- idempotency;
- concurrency behaviour.

Adapters that cannot satisfy a required guarantee must fail capability validation or configuration explicitly. They must not silently weaken semantics.

A reusable behavioral contract test suite must be executed against every implementation of a port. PostgreSQL and SQLite will be used as initial persistence compatibility references where appropriate; in-memory implementations are test doubles only unless explicitly accepted as production adapters.

Database migrations remain engine-specific infrastructure assets. Switching database engines may require a new schema and migration set, but must not change domain or application contracts.

## Consequences

### Positive

- Infrastructure providers can be replaced without rewriting use cases.
- REST, gRPC, workers and jobs share the same core regardless of provider.
- Provider-specific optimizations remain possible behind adapters.
- Contract tests expose semantic incompatibilities early.
- Vendor SDKs remain isolated.

### Negative

- More explicit ports and adapter crates are required.
- True portability requires discipline and contract testing.
- Some use cases may intentionally require capabilities unavailable in simpler backends.
- Database-engine migration is not reduced to changing one connection string; data conversion and engine-specific migrations remain operational work.

## Rejected alternatives

- Use one generic key-value or CRUD interface for every backend: rejected because it hides required business guarantees and creates a lowest-common-denominator abstraction.
- Expose SQLx, Redis or S3 types in application APIs: rejected because it couples the core to provider SDKs.
- Build a universal storage abstraction before use cases are known: rejected as speculative abstraction.

## Compliance

Architecture tests must reject infrastructure dependencies in domain and application crates. Every new adapter must pass the relevant shared contract suite. Exceptions require an accepted ADR or RFC.
