# Transport-Neutral Adapter Execution

- Status: Design baseline
- Governing issue: #51
- Governing decision: ADR-0028

## Purpose

Define how REST, gRPC, CLI, worker, scheduler and future delivery adapters expose the same canonical Operation without owning application behavior.

## Canonical flow

```text
protocol request or trigger
  -> adapter decode
  -> transport validation
  -> authentication evidence
  -> canonical context mapping
  -> Operation registry/dispatch
  -> typed Operation invocation
  -> canonical outcome and evidence
  -> protocol response or work disposition
```

The adapter boundary ends where canonical input and InvocationContext are produced. It resumes when the canonical outcome is encoded.

## Adapter responsibilities

An adapter may:

1. decode protocol representation;
2. enforce framing, syntax, media type, size and required transport fields;
3. authenticate using an approved mechanism and reference verified evidence;
4. derive approved InvocationContext fields;
5. select the requested registered Operation version, never a realization;
6. invoke the runtime dispatch port;
7. encode canonical output, error and disposition;
8. propagate safe correlation and diagnostic references.

An adapter must not:

- implement business rules or orchestration;
- access repositories for business work;
- open or control application transactions;
- call provider SDKs or select Capability Realizations;
- publish independent business effects;
- rewrite canonical errors based on transport convenience;
- inject unverified headers or claims into the context;
- create a transport-specific application service.

## Validation matrix

| Concern | Adapter | Operation/application |
|---|---:|---:|
| malformed JSON/protobuf/arguments | yes | no |
| media type, frame and size limits | yes | no |
| required protocol metadata | yes | no |
| canonical field type/range | mapper plus canonical validator | authoritative |
| business invariant | no | yes |
| authorization and resource policy | coarse admission only | authoritative |
| idempotency equivalence | no | authoritative |
| state/version conflict | no | authoritative |
| provider eligibility | no | Capability Binder/runtime |

A mapper must preserve field meaning, units, precision, nullability and ordering semantics. Lossy conversion is incompatible unless explicitly accepted by the Operation contract.

## Identity and execution context

Only verified or policy-derived facts enter InvocationContext. Each adapter documents the source and trust level for:

- actor/subject;
- tenant and legal entity;
- location, channel, touchpoint and device;
- locale, currency and jurisdiction;
- correlation and causation IDs;
- idempotency key;
- requested contract version;
- absolute deadline and cancellation token.

Client-supplied correlation IDs may be retained only after syntax/length validation and collision policy. Server-generated invocation IDs are always authoritative.

## Deadline and cancellation

- REST derives an absolute deadline from accepted request metadata or runtime policy.
- gRPC maps the protocol deadline directly, capped by server policy.
- CLI uses explicit timeout options or profile defaults.
- Worker adapters derive deadlines from the Work Envelope.
- Child calls cannot exceed the parent deadline.
- Disconnect or cancellation does not imply that business effects did not occur.

## Canonical disposition mapping

The normative mapping rule is semantic first, protocol second. Suggested defaults may be overridden only by an accepted adapter profile that preserves meaning.

| Canonical disposition/class | REST view | gRPC view | CLI view | Worker view |
|---|---|---|---|---|
| succeeded | 2xx | OK | exit 0 | completed |
| accepted | 202 plus handle | OK/accepted detail | exit 0 plus handle | accepted/completed durability state |
| invalid_input | 400/422 | INVALID_ARGUMENT | usage/data exit | dead-letter or rejected input |
| not_authorized | 401/403 | UNAUTHENTICATED/PERMISSION_DENIED | auth exit | rejected/quarantined |
| business_rejection | 409/422 | FAILED_PRECONDITION | business rejection exit | rejected |
| conflict | 409 | ABORTED | conflict exit | retry only by policy |
| dependency_unavailable | 503 | UNAVAILABLE | temporary failure exit | bounded retry/defer |
| timed_out | 504 | DEADLINE_EXCEEDED | timeout exit | retry/reconcile by certainty |
| cancelled | client-specific | CANCELLED | interrupted exit | cancelled |
| indeterminate | 202/409/5xx with reconciliation reference | UNKNOWN/ABORTED with safe detail | dedicated indeterminate exit | reconcile; never blind retry |
| internal_failure | 500 | INTERNAL | software failure exit | bounded retry/quarantine |

Exact status choice is declared in the adapter binding descriptor. It cannot expose provider identities or redefine retryability.

## REST binding

A REST binding declares method, path template, request media type, canonical mapper, response views, supported Operation versions and approved headers. Path design is not Operation identity. Conditional requests and idempotency headers are mapped only when the canonical contract supports them.

## gRPC binding

A gRPC binding declares service/method, protobuf revisions, canonical mapper and safe status details. Protobuf unknown-field behavior and numeric precision must be compatibility-tested. Streaming requires an accepted Operation semantic need and is not inferred from protocol capability.

## CLI binding

A CLI binding declares command path, argument/input mapping, output formats and exit-code table. Machine-readable output uses a stable schema. Terminal prompts and formatting stay outside canonical input/output semantics.

## Worker binding

A worker binding declares accepted Work Envelope/event revisions, canonical input mapper, acknowledgement durability point and allowed work dispositions. Retry, lease and quarantine behavior follow ADR-0027 and canonical Operation retry/idempotency semantics.

## Versioning

Each binding records:

- `adapter_id` and adapter revision;
- adapter kind;
- Operation ID and supported semantic-version range;
- input/output/error mapper revisions;
- runtime-profile compatibility;
- deprecation and sunset policy;
- canonical fixture references.

An adapter revision can evolve independently where canonical meaning is preserved. Unsupported combinations fail before Operation execution when feasible.

## Fixture strategy

For every proving Operation:

1. canonical input/output/error fixtures define business meaning once;
2. each adapter has representation fixtures mapping to those canonical fixtures;
3. all adapters assert the same canonical outcome;
4. adapter tests assert only protocol-specific encoding and status details;
5. native/delegated realization changes do not change adapter fixtures unless canonical semantics change.

## Observability and economics

Evidence links adapter kind/revision, invocation, Operation version, mapping outcome, runtime profile, realization and final disposition. Metrics use bounded adapter dimensions. Request payloads, credentials and customer/payment data are not logged by default.

Adapter overhead—decode, validation, mapping, encoding, bytes and CPU—is separately attributable from Operation and realization cost.

## Architecture enforcement

Dependency rules permit:

```text
adapter -> canonical contracts -> runtime invocation port
```

They forbid:

```text
adapter -> repository
adapter -> concrete provider
adapter -> transaction manager
adapter -> domain event publisher
adapter -> application implementation internals
```

## Acceptance evidence

M1 must include at least two of REST, gRPC, CLI or worker for the same Operation, shared canonical fixtures, status/error mapping tests, context propagation, architecture dependency checks and correlated performance/economic evidence.