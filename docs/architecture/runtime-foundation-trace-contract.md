# Trace propagation contract

Inbound adapters may accept W3C `traceparent` and `tracestate` values and map
them into the transport-neutral `TraceContext`. The REST gateway forwards the
same values as gRPC metadata. Operations receive only canonical strings and do
not initialize or access a global tracer.

The composition root remains responsible for attaching an OpenTelemetry SDK or
a deterministic test sink. Business payloads, secrets and provider SDK errors
must not be emitted as span attributes. Safe identifiers are operation ID,
tenant, correlation, realization/provider attribution and status category.
