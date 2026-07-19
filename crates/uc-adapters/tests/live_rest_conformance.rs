//! Live-process semantic conformance tests for the REST-to-gRPC delivery path.
//!
//! These tests start both real listeners and cross the complete HTTP → gateway → gRPC
//! → canonical Operation path. They prove that REST remains only a delivery adapter:
//! canonical response semantics must match direct and gRPC invocation, while safe
//! correlation and W3C trace context survive both transport boundaries.
//!
//! The first scenario is intentionally RED until the gateway copies safe response
//! metadata from gRPC into the outgoing HTTP response headers.

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};
use uc_adapters::{
    proto::runtime_service_client::RuntimeServiceClient, serve_gateway, serve_grpc,
};

const CORRELATION_HEADER: &str = "x-correlation-id";
const TRACEPARENT_HEADER: &str = "traceparent";
const CORRELATION_ID: &str = "rest-live-success";
const TRACEPARENT: &str = "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01";

#[tokio::test]
async fn success_preserves_canonical_response_correlation_and_trace_headers() {
    let grpc_address = reserve_loopback_address();
    let grpc_server = tokio::spawn(async move { serve_grpc(grpc_address).await });
    wait_for_grpc(grpc_address).await;

    let gateway_address = reserve_loopback_address();
    let grpc_endpoint = format!("http://{grpc_address}");
    let gateway_server =
        tokio::spawn(async move { serve_gateway(gateway_address, grpc_endpoint).await });

    let response = tokio::task::spawn_blocking(move || request_gateway_with_retry(gateway_address))
        .await
        .expect("blocking HTTP client task must complete");

    gateway_server.abort();
    grpc_server.abort();

    let (head, body) = response
        .split_once("\r\n\r\n")
        .expect("HTTP response must contain a header/body separator");
    assert!(head.starts_with("HTTP/1.1 200"), "unexpected response: {head}");
    assert!(
        head.lines().any(|line| {
            line.eq_ignore_ascii_case(&format!("{CORRELATION_HEADER}: {CORRELATION_ID}"))
        }),
        "REST success responses must expose the safe correlation identifier"
    );
    assert!(
        head.lines().any(|line| {
            line.eq_ignore_ascii_case(&format!("{TRACEPARENT_HEADER}: {TRACEPARENT}"))
        }),
        "REST success responses must preserve the inbound W3C trace context"
    );

    let json: serde_json::Value =
        serde_json::from_str(body).expect("gateway response body must be valid JSON");
    assert_eq!(json["message"], "hello");
    assert_eq!(json["tenant_id"], "tenant-a");
    assert_eq!(json["correlation_id"], CORRELATION_ID);
}

fn reserve_loopback_address() -> SocketAddr {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("the test runner must provide an ephemeral loopback port");
    listener
        .local_addr()
        .expect("the reserved loopback listener must have a local address")
}

async fn wait_for_grpc(address: SocketAddr) {
    let endpoint = format!("http://{address}");
    for _ in 0..50 {
        if RuntimeServiceClient::connect(endpoint.clone()).await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("live gRPC server did not become ready at {endpoint}");
}

fn request_gateway_with_retry(address: SocketAddr) -> String {
    for _ in 0..50 {
        if let Ok(response) = request_gateway(address) {
            return response;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    panic!("live REST gateway did not become ready at {address}");
}

fn request_gateway(address: SocketAddr) -> std::io::Result<String> {
    let body = format!(
        r#"{{"message":" hello ","tenant_id":"tenant-a","identity":"live-rest-test","correlation_id":"{CORRELATION_ID}"}}"#
    );
    let request = format!(
        "POST /v1/ping HTTP/1.1\r\nHost: {address}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n{TRACEPARENT_HEADER}: {TRACEPARENT}\r\n\r\n{body}",
        body.len()
    );

    let mut stream = TcpStream::connect_timeout(&address, Duration::from_millis(100))?;
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}
