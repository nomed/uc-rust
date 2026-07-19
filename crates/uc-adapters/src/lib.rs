//! gRPC delivery adapter and REST-to-gRPC gateway for the Runtime Foundation.
//!
//! This crate is the transport boundary. It owns protobuf generation exposure,
//! request/response mapping, W3C trace and timeout propagation, native status mapping,
//! and process listeners. Canonical Operations remain transport-neutral and are the
//! sole source of business semantics. Public responses must not expose provider SDK
//! errors, stack details, secrets, or protected business payloads.

/// Generated protobuf types for the versioned runtime delivery contract.
///
/// These types are confined to this adapter crate and must not leak into canonical
/// application or domain APIs.
pub mod proto {
    tonic::include_proto!("uc.runtime.v1");
}

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
};
use proto::{
    runtime_service_client::RuntimeServiceClient,
    runtime_service_server::{RuntimeService, RuntimeServiceServer},
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};
use tonic::{Request, Response, Status, metadata::MetadataValue};
use uc_operation::{
    CancellationToken, ExecutionContext, Operation, OperationError,
    PingRequest as CanonicalPingRequest, TraceContext,
};
use uc_runtime::PingOperation;

const CORRELATION_HEADER: &str = "x-correlation-id";
const TRACEPARENT_HEADER: &str = "traceparent";

/// gRPC server implementation that maps protobuf requests into canonical Operations.
#[derive(Clone, Default)]
pub struct GrpcRuntimeService {
    operation: PingOperation,
}

#[tonic::async_trait]
impl RuntimeService for GrpcRuntimeService {
    async fn ping(
        &self,
        request: Request<proto::PingRequest>,
    ) -> Result<Response<proto::PingResponse>, Status> {
        let traceparent = metadata_string(request.metadata(), TRACEPARENT_HEADER);
        let response_traceparent = traceparent.clone();
        let tracestate = metadata_string(request.metadata(), "tracestate");
        let timeout = metadata_string(request.metadata(), "x-timeout-ms")
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_millis)
            .unwrap_or_else(|| Duration::from_secs(30));
        let request = request.into_inner();
        let correlation_id = request.correlation_id.clone();
        let response = self
            .operation
            .execute(
                CanonicalPingRequest {
                    message: request.message,
                },
                ExecutionContext {
                    tenant_id: request.tenant_id,
                    identity: request.identity,
                    correlation_id: request.correlation_id,
                    idempotency_key: (!request.idempotency_key.is_empty())
                        .then_some(request.idempotency_key),
                    trace: TraceContext {
                        traceparent,
                        tracestate,
                    },
                    deadline: Some(std::time::Instant::now() + timeout),
                    cancellation: CancellationToken::default(),
                },
            )
            .await
            .map_err(|error| map_error(error, &correlation_id))?;

        let mut response = Response::new(proto::PingResponse {
            message: response.message,
            tenant_id: response.tenant_id,
            correlation_id: response.correlation_id,
        });
        insert_response_metadata(&mut response, CORRELATION_HEADER, &correlation_id);
        if let Some(traceparent) = response_traceparent.as_deref() {
            insert_response_metadata(&mut response, TRACEPARENT_HEADER, traceparent);
        }
        Ok(response)
    }
}

fn metadata_string(metadata: &tonic::metadata::MetadataMap, key: &'static str) -> Option<String> {
    metadata
        .get(key)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
}

fn insert_response_metadata<T>(response: &mut Response<T>, key: &'static str, value: &str) {
    if let Ok(value) = MetadataValue::try_from(value) {
        response.metadata_mut().insert(key, value);
    }
}

/// Serves the versioned gRPC runtime API until the server terminates.
///
/// The listener exposes delivery mapping only; canonical semantics are delegated to
/// `uc-runtime` Operations.
pub async fn serve_grpc(addr: SocketAddr) -> Result<(), tonic::transport::Error> {
    tonic::transport::Server::builder()
        .add_service(RuntimeServiceServer::new(GrpcRuntimeService::default()))
        .serve(addr)
        .await
}

#[derive(Clone)]
struct GatewayState {
    grpc_endpoint: String,
}

#[derive(Debug, Deserialize)]
struct GatewayPingRequest {
    message: String,
    tenant_id: String,
    identity: String,
    correlation_id: String,
    #[serde(default)]
    idempotency_key: String,
}

#[derive(Debug, Serialize)]
struct GatewayPingResponse {
    message: String,
    tenant_id: String,
    correlation_id: String,
}

#[derive(Debug, Serialize)]
struct ProblemDetails {
    title: String,
    status: u16,
    correlation_id: Option<String>,
}

/// Serves the REST/JSON gateway that delegates every request to the gRPC adapter.
///
/// Trace context and timeout metadata are propagated to gRPC. Transport failures are
/// mapped into safe problem details without exposing internal implementation errors.
pub async fn serve_gateway(addr: SocketAddr, grpc_endpoint: String) -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/v1/ping", post(gateway_ping))
        .with_state(GatewayState { grpc_endpoint });
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await
}

async fn gateway_ping(
    State(state): State<GatewayState>,
    headers: HeaderMap,
    Json(request): Json<GatewayPingRequest>,
) -> Result<Json<GatewayPingResponse>, (StatusCode, Json<ProblemDetails>)> {
    let correlation_id = request.correlation_id.clone();
    let mut client = RuntimeServiceClient::connect(state.grpc_endpoint)
        .await
        .map_err(|_| {
            gateway_problem(
                StatusCode::BAD_GATEWAY,
                "gRPC service unavailable",
                Some(correlation_id.clone()),
            )
        })?;
    let mut grpc_request = Request::new(proto::PingRequest {
        message: request.message,
        tenant_id: request.tenant_id,
        identity: request.identity,
        correlation_id,
        idempotency_key: request.idempotency_key,
    });
    copy_header_to_metadata(&headers, &mut grpc_request, TRACEPARENT_HEADER);
    copy_header_to_metadata(&headers, &mut grpc_request, "tracestate");
    copy_header_to_metadata(&headers, &mut grpc_request, "x-timeout-ms");

    let response = client
        .ping(grpc_request)
        .await
        .map_err(|status| grpc_status_problem(status, Some(request.correlation_id.clone())))?
        .into_inner();

    Ok(Json(GatewayPingResponse {
        message: response.message,
        tenant_id: response.tenant_id,
        correlation_id: response.correlation_id,
    }))
}

fn copy_header_to_metadata(
    headers: &HeaderMap,
    request: &mut Request<proto::PingRequest>,
    key: &'static str,
) {
    if let Some(value) = headers.get(key).and_then(|value| value.to_str().ok()) {
        if let Ok(value) = MetadataValue::try_from(value) {
            request.metadata_mut().insert(key, value);
        }
    }
}

fn gateway_problem(
    status: StatusCode,
    title: &str,
    correlation_id: Option<String>,
) -> (StatusCode, Json<ProblemDetails>) {
    (
        status,
        Json(ProblemDetails {
            title: title.into(),
            status: status.as_u16(),
            correlation_id,
        }),
    )
}

fn grpc_status_problem(
    status: Status,
    correlation_id: Option<String>,
) -> (StatusCode, Json<ProblemDetails>) {
    let http = match status.code() {
        tonic::Code::InvalidArgument => StatusCode::BAD_REQUEST,
        tonic::Code::Unauthenticated => StatusCode::UNAUTHORIZED,
        tonic::Code::PermissionDenied => StatusCode::FORBIDDEN,
        tonic::Code::NotFound => StatusCode::NOT_FOUND,
        tonic::Code::AlreadyExists | tonic::Code::Aborted => StatusCode::CONFLICT,
        tonic::Code::DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,
        tonic::Code::Cancelled => StatusCode::REQUEST_TIMEOUT,
        tonic::Code::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    gateway_problem(http, status.message(), correlation_id)
}

fn map_error(error: OperationError, correlation_id: &str) -> Status {
    let mut status = match error {
        OperationError::InvalidRequest(message) => Status::invalid_argument(message),
        OperationError::Unauthorized => Status::unauthenticated("unauthorized"),
        OperationError::Forbidden => Status::permission_denied("forbidden"),
        OperationError::NotFound => Status::not_found("not found"),
        OperationError::Conflict(message) => Status::already_exists(message),
        OperationError::DeadlineExceeded => Status::deadline_exceeded("deadline exceeded"),
        OperationError::Cancelled => Status::cancelled("cancelled"),
        OperationError::Unavailable => Status::unavailable("service unavailable"),
        OperationError::Internal => Status::internal("internal operation failure"),
    };
    if let Ok(value) = MetadataValue::try_from(correlation_id) {
        status.metadata_mut().insert(CORRELATION_HEADER, value);
    }
    status
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_errors_map_deterministically() {
        assert_eq!(
            map_error(OperationError::DeadlineExceeded, "corr").code(),
            tonic::Code::DeadlineExceeded
        );
        assert_eq!(
            map_error(OperationError::Cancelled, "corr").code(),
            tonic::Code::Cancelled
        );
        assert_eq!(
            map_error(OperationError::Forbidden, "corr").code(),
            tonic::Code::PermissionDenied
        );
    }
}
