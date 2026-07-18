# Operation Invocation Test Plan

- Status: Design baseline
- Governing issue: #47
- Governing decision: ADR-0021
- Related: ADR-0024, #49, #51, #54

## Objective

Prove that every delivery mechanism invokes the same canonical Operation contract and that invocation semantics remain correct across runtime profiles and Capability Realizations.

## Test layers

### 1. Contract-schema tests

Validate every Operation manifest against `governance/schemas/operation-manifest.schema.json` and verify that referenced input, output and error schemas exist and are versioned.

Required assertions:

- stable Operation ID format;
- semantic version present;
- capability and owner present;
- explicit invocation, authorization, idempotency, transaction and effect policies;
- supported profiles declared;
- observability and economic attribution declared;
- fixtures and architecture-test identifiers present.

### 2. Canonical fixture tests

Execute human-readable fixtures directly against the typed Operation implementation.

Each fixture records:

- Operation ID and semantic version;
- normalized invocation context;
- canonical input;
- expected output or error disposition;
- expected state changes and effects;
- expected idempotency disposition;
- expected trace/economic observations without sensitive payloads.

### 3. Multi-adapter equivalence

Run the same fixture set through at least two adapters, initially REST plus CLI or worker.

Compare canonical normalized results, not transport-specific envelopes. The test fails if an adapter introduces a private business error, different default, altered authorization rule or independent side effect.

### 4. Context propagation

Verify:

- tenant, actor, store and runtime profile remain unchanged;
- unapproved transport headers do not enter application context;
- correlation and causation identifiers propagate;
- child deadlines never exceed parent deadline;
- cancellation reaches declared safe points;
- policy/configuration revisions are observable;
- sensitive identity evidence is referenced rather than copied to telemetry.

### 5. Authorization

Verify deny-by-default behavior, resource derivation, cross-tenant rejection, audit evidence and any declared post-state checks. Adapter authentication alone must not bypass Operation authorization.

### 6. Idempotency and concurrency

For mutating Operations test:

- first invocation;
- sequential duplicate;
- concurrent duplicate;
- same key with non-equivalent input;
- expiry of retention window;
- restart during in-progress disposition;
- delegated-provider indeterminate outcome;
- replay of canonical success or rejection.

Blind retry after `indeterminate` must be rejected unless reconciliation policy explicitly authorizes it.

### 7. Transaction and effects

For `local_atomic` Operations inject failures before and after each persistence/effect boundary and prove:

- state, idempotency record and required outbox effects commit atomically;
- no adapter publishes an independent business effect;
- post-commit work is distinguishable from atomic effects;
- restart/replay does not duplicate business outcomes.

For `saga` and `external_authority`, prove explicit compensation or reconciliation states rather than simulated atomicity.

### 8. Error taxonomy and retryability

Map domain, repository, runtime and provider failures into stable canonical classes. Verify transport mappings separately.

Required scenarios:

- invalid input;
- not authorized;
- business rejection;
- conflict;
- dependency unavailable;
- deadline exceeded before and after possible external effect;
- safe cancellation;
- indeterminate outcome;
- unsupported profile/version;
- internal failure.

Every error fixture declares whether retry, rebind, reconcile, change-input or human intervention is allowed.

### 9. Capability Realization equivalence

For an Operation using ADR-0024, run the same semantic fixtures against native and delegated realizations. Provider-specific diagnostics may differ; canonical results, invariants and stable errors must remain equivalent within declared capability limits.

### 10. Observability and Economics by Design

Verify each invocation emits:

- Operation and semantic version;
- selected realization;
- final disposition and canonical error class;
- duration and retry/deadline observations;
- resource/economic attribution unit;
- effect and reconciliation counts;
- no sensitive payload fields by default.

### 11. Architecture enforcement

Run dependency and source-boundary checks defined in `docs/architecture/operation-architecture-enforcement.md`.

## Initial proving Operation

`uc-rust:basket.create` is the initial contract example. M1 may replace it with another Operation only through an explicit gate update preserving all evidence categories.

Required initial proof:

```text
same canonical fixture
  -> REST adapter
  -> CLI or worker adapter
  -> one typed Operation
  -> one local atomic unit of work
  -> canonical outcome and outbox evidence
```

The delegated-realization proof may use commercial calculation in M1/M3, but must reuse this invocation contract.

## Exit criteria

#47 implementation evidence is complete when:

- all Operation manifests validate;
- one Operation passes direct and two-adapter fixture suites;
- deadline, cancellation, authorization, idempotency, transaction and effect tests pass;
- canonical errors and retry dispositions are verified;
- architecture boundary checks pass;
- trace and economic evidence is reproducible;
- no delivery adapter or worker requires a private business-service contract.