pub mod proto {
    tonic::include_proto!("uc.runtime.v1");
}

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use proto::{
    runtime_service_client::RuntimeServiceClient,
    runtime_service_server::{RuntimeService, RuntimeServiceServer},
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};
use tonic::{metadata::MetadataValue, Request, Response, Status};
use uc_operation::{
    CancellationToken, ExecutionContext, Operation, OperationError,
    PingRequest as CanonicalPingRequest, TraceContext,
};
use uc_runtime::PingOperation;

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
        let traceparent = metadata_string(request.metadata(), "traceparent");
        let tracestate = metadata_string(request.metadata(), "tracestate");
        let timeout = metadata_string(request.metadata(), "x-timeout-ms")
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_millis)
            .unwrap_or_else(|| Duration::from_secs(30));
        let request = request.into_inner();
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
            .map_err(map_error)?;

        Ok(Response::new(proto::PingResponse {
            message: response.message,
            tenant_id: response.tenant_id,
            correlation_id: response.correlation_id,
        }))
    }
}

fn metadata_string(metadata: &tonic::metadata::MetadataMap, key: &'static str) -> Option<String> {
    metadata
        .get(key)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
}

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

pub async fn serve_gateway(
    addr: SocketAddr,
    grpc_endpoint: String,
) -> Result<(), std::io::Error> {
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
        .map_err(|_| gateway_problem(StatusCode::BAD_GATEWAY, "gRPC service unavailable", Some(correlation_id.clone())))?;
    let mut grpc_request = Request::new(proto::PingRequest {
        message: request.message,
        tenant_id: request.tenant_id,
        identity: request.identity,
        correlation_id,
        idempotency_key: request.idempotency_key,
    });
    copy_header_to_metadata(&headers, &mut grpc_request, "traceparent");
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

fn map_error(error: OperationError) -> Status {
    match error {
        OperationError::InvalidRequest(message) => Status::invalid_argument(message),
        OperationError::Unauthorized => Status::unauthenticated("unauthorized"),
        OperationError::Forbidden => Status::permission_denied("forbidden"),
        OperationError::NotFound => Status::not_found("not found"),
        OperationError::Conflict(message) => Status::already_exists(message),
        OperationError::DeadlineExceeded => Status::deadline_exceeded("deadline exceeded"),
        OperationError::Cancelled => Status::cancelled("cancelled"),
        OperationError::Unavailable => Status::unavailable("service unavailable"),
        OperationError::Internal => Status::internal("internal operation failure"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_errors_map_deterministically() {
        assert_eq!(map_error(OperationError::DeadlineExceeded).code(), tonic::Code::DeadlineExceeded);
        assert_eq!(map_error(OperationError::Cancelled).code(), tonic::Code::Cancelled);
        assert_eq!(map_error(OperationError::Forbidden).code(), tonic::Code::PermissionDenied);
    }
}
