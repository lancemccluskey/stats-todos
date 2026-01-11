use mccluskeystack_homelab_schemas::homelab::stats::todos::v1::{
    GetTodosStatsRequest, GetTodosStatsResponse, HealthCheckRequest, HealthCheckResponse,
    HealthCheckStatus, UploadTodosRequest, UploadTodosResponse, todos_service_server::TodosService,
};
use tonic::{Request, Response, Result, Status, Streaming};
use tracing::debug;

pub struct Service;

impl Service {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl TodosService for Service {
    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        debug!("processing health_check");
        Ok(Response::new(HealthCheckResponse {
            status: HealthCheckStatus::Up as i32,
            ..Default::default()
        }))
    }

    async fn get_todos_stats(
        &self,
        _request: Request<GetTodosStatsRequest>,
    ) -> Result<Response<GetTodosStatsResponse>, Status> {
        debug!("processing get_todos_stats");
        Err(Status::new(
            tonic::Code::Unimplemented,
            "unimplemented".to_string(),
        ))
    }

    async fn upload_todos(
        &self,
        _request: Request<Streaming<UploadTodosRequest>>,
    ) -> Result<Response<UploadTodosResponse>, Status> {
        debug!("processing upload_todos");
        Err(Status::new(
            tonic::Code::Unimplemented,
            "unimplemented".to_string(),
        ))
    }
}
