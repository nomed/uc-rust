# Release gates through 1.0

Each release gate requires evidence in six dimensions:

1. **Functional** — canonical operations and expected outcomes.
2. **Architectural** — dependency boundaries and one-place business logic.
3. **Distributed** — consistency, offline and recovery behavior where applicable.
4. **Security** — authentication, authorization, isolation and supply-chain evidence.
5. **Operational** — health, observability, rollback and incident reproduction.
6. **Economic** — resource consumption, attributable cost and regression evidence.

A release cannot be marked complete from issue closure alone. The governing epic must link the accepted evidence bundle and state any approved exception with owner and expiry.
