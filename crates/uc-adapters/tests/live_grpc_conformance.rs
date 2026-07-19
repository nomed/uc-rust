//! Live-process semantic conformance tests for the gRPC delivery adapter.
//!
//! These tests cross the real tonic transport boundary rather than invoking adapter
//! functions directly. They prove that transport decoding, canonical Operation
//! execution and transport-native error mapping preserve the same semantics and safe
//! diagnostic context. The suite owns no business rules and must reuse canonical
//! Runtime Foundation fixtures as it grows.
//!
//! The first test captures the initial red-green cycle: invalid canonical input must
//! return `InvalidArgument` and preserve the safe correlation identifier in response
//! metadata.

use std::{net::SocketAddr, time::Duration};
use tonic::Request;
use uc_adapters::{
    proto::{PingRequest, runtime_service_client::RuntimeServiceClient},
    serve_grpc,
};

const CORRELATION_HEADER: &str = "x-correlation-id";
const CORRELATION_ID: &str = "grpc-live-invalid-input";

#[tokio::test]
async fn invalid_input_preserves_safe_correlation_metadata() {
    let address = reserve_loopback_address();
    let server = tokio::spawn(async move { serve_grpc(address).await });
    let mut client = connect_with_retry(address).await;

    let response = client
        .ping(Request::new(PingRequest {
            message: "   ".into(),
            tenant_id: "tenant-a".into(),
            identity: "live-grpc-test".into(),
            correlation_id: CORRELATION_ID.into(),
            idempotency_key: String::new(),
        }))
        .await;

    server.abort();

    let status = response.expect_err("blank Ping input must be rejected");
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
    assert_eq!(
        status
            .metadata()
            .get(CORRELATION_HEADER)
            .and_then(|value| value.to_str().ok()),
        Some(CORRELATION_ID),
        "public gRPC failures must preserve the safe correlation identifier"
    );
}

fn reserve_loopback_address() -> SocketAddr {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("the test runner must provide an ephemeral loopback port");
    listener
        .local_addr()
        .expect("the reserved loopback listener must have a local address")
}

async fn connect_with_retry(
    address: SocketAddr,
) -> RuntimeServiceClient<tonic::transport::Channel> {
    let endpoint = format!("http://{address}");
    for _ in 0..50 {
        match RuntimeServiceClient::connect(endpoint.clone()).await {
            Ok(client) => return client,
            Err(_) => tokio::time::sleep(Duration::from_millis(10)).await,
        }
    }
    panic!("live gRPC server did not become ready at {endpoint}");
}
