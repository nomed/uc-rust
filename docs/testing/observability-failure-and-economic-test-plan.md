# Observability, Failure, Health and Economic Evidence Test Plan

- Governing issue: #49
- Governing decision: ADR-0026

## Objectives

Prove that Operation outcomes can be diagnosed and economically attributed without coupling application/domain code to an observability vendor, leaking protected data or creating unbounded telemetry cost.

## Contract tests

1. Evidence Envelope validates for every supported evidence type.
2. Required correlation, Operation, runtime and classification fields are always present.
3. Provider-specific diagnostics do not cross the canonical Operation error boundary.
4. Application/domain crates compile without exporter or vendor SDK dependencies.
5. Replacing one exporter with another changes no Operation contract or fixture.

## Outcome evidence matrix

For one canonical Operation, produce and reconstruct:

- succeeded;
- rejected;
- failed;
- timed out;
- cancelled;
- indeterminate.

Each case must link invocation, Operation, runtime profile, active configuration revision, realization/binding where applicable, canonical disposition, health impact and economic attribution completeness.

## Native/delegated tests

- Execute the same semantic fixture through native and delegated realizations.
- Confirm identical canonical outcome meaning.
- Confirm distinct realization/provider attribution.
- Confirm timeout, provider failure and indeterminate outcome remain visible.
- Confirm provider identifiers do not appear in the consumer contract.

## Failure Bundle tests

- Reconstruct the failure cause from bounded evidence.
- Verify retryability and reconciliation disposition.
- Verify configuration, contract and binding revisions.
- Verify runbook/owner references.
- Verify secrets, tokens, payment data and customer payloads are absent.
- Verify public error view and protected diagnostic view differ appropriately.

## Health tests

- Live but not ready runtime.
- Ready runtime with optional dependency degraded.
- Store edge operational for local subset while WAN and delegated provider are unavailable.
- Fiscal/legal authority unavailable blocks only affected Operations and journey.
- Expired health observation becomes unknown, never healthy.
- Journey health reports checkout unavailable while basket remains operational.
- Aggregate status cannot mask a security, authority or legal blocker.

## Cardinality and privacy tests

- Reject prohibited high-cardinality metric labels.
- Enforce configured maximum series/cardinality budgets.
- Verify tenant, invocation, basket, customer, order and device identities are not metric labels.
- Verify identifiers in traces/events are minimized or tokenized.
- Verify payload logging is denied by default.
- Verify retention and access classification are applied.

## Sampling and backpressure tests

- Successful traces may be sampled according to policy.
- Errors, indeterminate outcomes, security events and mandatory economic reconciliation evidence are retained.
- Exporter outage does not create unbounded memory.
- Drop/sampling counts are themselves observable.
- Mandatory legal/audit sink failure causes the declared safe block rather than silent evidence loss.

## Economic evidence tests

- Attribute duration, CPU, allocations, network, storage and provider units where measurable.
- Link observation to economic unit, allocation rule and model revision.
- Mark incomplete dimensions as partial/unavailable rather than inventing values.
- Aggregate by Operation, realization, profile and accepted economic unit.
- Verify economic observation does not alter the business outcome.
- Compare native and delegated realization cost-to-serve using the same semantic fixture.

## Performance and cost budgets

Measure:

- invocation overhead with telemetry disabled, minimal and full;
- CPU and allocation overhead;
- queue/buffer memory;
- exported bytes per outcome;
- storage per retained evidence class;
- provider telemetry cost;
- cardinality growth under representative tenant/store/Operation load.

A budget breach is explicit gate evidence and cannot be hidden by reducing mandatory diagnostic, security, legal or economic evidence.

## Architecture enforcement

Automated dependency tests must prevent:

- application/domain imports of exporter/vendor SDKs;
- adapters inventing canonical business errors;
- direct logging of canonical request/response payloads;
- health contributors mutating application state;
- economic exporters becoming transaction authorities;
- metric APIs accepting unbounded arbitrary labels.

## M1/M8 evidence bundle

The accepted evidence bundle contains fixtures, test output, schema validation, dependency graph, redaction report, health matrix, cardinality report, performance benchmark and economic attribution report.