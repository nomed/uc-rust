pub mod proto {
    tonic::include_proto!("uc.runtime.v1");
}

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use proto::{
    runtime_service_client::RuntimeServiceClient,
    runtime_service_server::{RuntimeService, RuntimeServiceServer},
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};
use tonic::{Request, Response, Status};
use uc_operation::{
    ExecutionContext, Operation, OperationError, PingRequest as CanonicalPingRequest,
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
                    deadline: Some(std::time::Instant::now() + Duration::from_secs(30)),
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
    Json(request): Json<GatewayPingRequest>,
) -> Result<Json<GatewayPingResponse>, (StatusCode, String)> {
    let mut client = RuntimeServiceClient::connect(state.grpc_endpoint)
        .await
        .map_err(|error| (StatusCode::BAD_GATEWAY, error.to_string()))?;
    let response = client
        .ping(proto::PingRequest {
            message: request.message,
            tenant_id: request.tenant_id,
            identity: request.identity,
            correlation_id: request.correlation_id,
            idempotency_key: request.idempotency_key,
        })
        .await
        .map_err(|error| (StatusCode::BAD_GATEWAY, error.to_string()))?
        .into_inner();

    Ok(Json(GatewayPingResponse {
        message: response.message,
        tenant_id: response.tenant_id,
        correlation_id: response.correlation_id,
    }))
}

fn map_error(error: OperationError) -> Status {
    match error {
        OperationError::InvalidRequest(message) => Status::invalid_argument(message),
        OperationError::DeadlineExceeded => Status::deadline_exceeded("deadline exceeded"),
        OperationError::Cancelled => Status::cancelled("cancelled"),
        OperationError::Internal => Status::internal("internal operation failure"),
    }
}
