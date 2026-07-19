//! gRPC delivery adapter for the Runtime Foundation.
//!
//! This crate owns protobuf exposure, request/response mapping, W3C trace and timeout
//! propagation, native gRPC status mapping, and the gRPC listener. Canonical Operations
//! remain transport-neutral and are the sole source of business semantics. REST/JSON
//! transcoding is intentionally delegated to generated gRPC-Gateway infrastructure
//! tracked by issue #80 rather than reimplemented in Rust.

/// Generated protobuf types for the versioned runtime delivery contract.
///
/// These types are confined to this adapter crate and must not leak into canonical
/// application or domain APIs.
pub mod proto {
    tonic::include_proto!("uc.runtime.v1");
}

use proto::runtime_service_server::{RuntimeService, RuntimeServiceServer};
use std::{net::SocketAddr, time::Duration};
use tonic::{Request, Response, Status, metadata::MetadataValue};
use uc_operation::{
    CancellationToken, ExecutionContext, Operation, OperationError,
    PingRequest as CanonicalPingRequest, TraceContext,
};
use uc_runtime::PingOperation;

const CANCELLATION_HEADER: &str = "x-uc-cancelled";
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
        let cancellation = CancellationToken::default();
        if metadata_string(request.metadata(), CANCELLATION_HEADER).as_deref() == Some("true") {
            cancellation.cancel();
        }
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
                    cancellation,
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
