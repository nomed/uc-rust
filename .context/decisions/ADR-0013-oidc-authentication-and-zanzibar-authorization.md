# ADR-0013 — OIDC authentication and Zanzibar-style authorization

- Status: Accepted
- Date: 2026-07-18
- Governing issue: #36

## Context

UC Rust requires authentication for human users, services and agents, and fine-grained authorization across tenants, organizations, companies, locations, channels, stores and business resources. Authorization must remain centralized, consistent, testable, highly performant and independent from REST, gRPC, worker or job adapters.

Google Zanzibar defines a relationship-based authorization model with causal consistency properties suitable for shared resources and distributed systems. UC Rust must not implement an ad-hoc role system inside business handlers.

## Decision

Authentication uses provider-neutral OAuth 2.0 and OpenID Connect contracts. Interactive applications use Authorization Code with PKCE. Services use workload identity or Client Credentials only where appropriate. Token issuer, audience, signature, expiry and key rotation are validated explicitly.

Authorization uses a Zanzibar-style relationship model. The application core owns an `AuthorizationPort` expressed with domain resource identifiers, actions, subjects and required consistency. Delivery adapters authenticate and construct the principal context, but application operations own the authorization decision point.

SpiceDB is the initial authorization adapter because it provides:

- Zanzibar-inspired relationship and permission schemas;
- gRPC APIs and bulk permission checks;
- explicit consistency levels and ZedTokens for causal/read-after-write requirements;
- containerized local and CI execution;
- Kubernetes production operation through the SpiceDB Operator;
- metrics, tracing and performance-oriented deployment controls.

SpiceDB types, client messages, ZedToken representations and schema details must not leak into domain contracts. A future provider must be replaceable through a new adapter and the same authorization contract suite.

## Rules

- Deny by default.
- Every protected application operation declares subject, resource, permission and consistency requirement.
- RBAC is modeled through relationships and groups rather than hard-coded conditionals.
- Tenant isolation is represented structurally and tested against cross-tenant access.
- Relationship writes have explicit ownership, transaction coordination and recovery semantics.
- Authorization model changes are versioned, tested and migrated like database schema changes.
- Read-after-write sensitive workflows use causal consistency tokens or a stronger documented mode.
- Authorization failure must not silently degrade to allow.
- Local caches may not weaken the required consistency or revocation behavior.
- Audit evidence records decision identifiers without leaking sensitive token contents.

## Consequences

Authorization becomes an explicit platform capability and an external dependency in production. GitHub Actions must provision an ephemeral SpiceDB instance for model, contract, isolation and performance tests. The platform must define behavior when the authorization service is unavailable and must include authorization latency in performance budgets.

## Alternatives considered

### OpenFGA

A credible CNCF-hosted Zanzibar-inspired implementation with a readable DSL and strong ecosystem. It remains a valid future adapter. SpiceDB is selected initially because its explicit per-request consistency controls, ZedTokens and Kubernetes Operator align more closely with UC Rust's strict consistency and operability requirements.

### Ory Keto

A Zanzibar-inspired authorization service that integrates well with the broader Ory identity ecosystem. It was not selected as the initial adapter because UC Rust wants authentication-provider neutrality and does not currently intend to adopt the whole Ory stack.

### In-process RBAC or policy engine

Rejected as the primary authorization authority because it would encourage duplicated authorization logic, weak relationship modeling and inconsistent decisions across adapters and services.
