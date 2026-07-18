# Runtime Foundation architecture review checklist

The Runtime Foundation may enter implementation only when every item below is resolved or explicitly deferred by an accepted decision.

## Product and scope

- 1.0 reference journey and runtime responsibilities are explicit.
- Runtime non-goals prevent framework overreach.
- Central and edge profiles reuse the same application operations.

## Operation model

- Operation identity, input, output and error contracts are defined.
- Execution context, identity, deadline, cancellation and correlation are defined.
- Authorization and transaction-boundary ownership are explicit.
- Delivery adapters cannot bypass Operations.

## Runtime model

- Lifecycle state machine and graceful shutdown are defined.
- Configuration loading, validation, revision and reload semantics are defined.
- Composition avoids global mutable state and service locator access.
- Background work and scheduler guarantees are defined.
- Extension boundaries and 1.0 compatibility expectations are defined.

## Distributed and edge

- Runtime profile and capability discovery contracts are defined.
- Data-plane and control-plane concerns remain separate.
- No custom consensus is introduced without ADR evidence.
- WAN partition behavior and local readiness semantics are explicit.

## Quality and economics

- Latency, allocation, idle CPU and memory budgets exist.
- Logging, tracing, metrics, health and error evidence contracts exist.
- Economic Unit and consumption-driver correlation are defined.
- Test strategy includes unit, property, contract, integration and failure proof.
- Component choices include a cost-to-serve scorecard.

## Governance

- ADR-0021 accepted.
- RFC-0001 accepted.
- New decisions discovered during design are recorded.
- UC-BoK and EbD impacts are mapped.
- Child issues, dependencies and exit evidence are represented in GitHub/Yukh.
