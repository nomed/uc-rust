//! Deterministic tracing contract for the gRPC delivery adapter.
//!
//! This fixture invokes the adapter directly while installing an isolated tracing
//! subscriber. It records the required transport-neutral hierarchy before production
//! instrumentation exists: decode → invocation → operation → encode. Span names are
//! stable public diagnostic vocabulary and must carry only safe identifiers.

use tonic::Request;
use tracing_test::traced_test;
use uc_adapters::{
    GrpcRuntimeService,
    proto::{PingRequest, runtime_service_server::RuntimeService},
};

const CORRELATION_ID: &str = "trace-hierarchy-correlation";

#[tokio::test]
#[traced_test]
async fn ping_emits_required_span_vocabulary_and_safe_correlation() {
    let service = GrpcRuntimeService::default();
    let response = service
        .ping(Request::new(PingRequest {
            message: " hello ".into(),
            tenant_id: "tenant-a".into(),
            identity: "trace-test".into(),
            correlation_id: CORRELATION_ID.into(),
            idempotency_key: String::new(),
        }))
        .await
        .expect("valid Ping invocation must succeed");

    assert_eq!(response.get_ref().message, "hello");
    assert!(logs_contain("decode"), "delivery decoding span is required");
    assert!(
        logs_contain("invocation"),
        "one invocation span must contain the request lifecycle"
    );
    assert!(
        logs_contain("operation"),
        "canonical Operation execution span is required"
    );
    assert!(logs_contain("encode"), "delivery encoding span is required");
    assert!(
        logs_contain(CORRELATION_ID),
        "safe correlation identifier must be attached to diagnostic spans"
    );
}
