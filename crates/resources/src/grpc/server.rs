use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use anyhow::Result;
use mccluskeystack_homelab_schemas::homelab::stats::todos::v1::todos_service_server::{
    TodosService, TodosServiceServer,
};
use tonic_web::GrpcWebLayer;
use tracing::debug;

use crate::grpc::config::Config;

pub struct Server<Svc> {
    svc: Svc,
    addr: SocketAddr,
}

impl<Svc> Server<Svc>
where
    Svc: TodosService,
{
    pub fn new(svc: Svc, cfg: Config) -> Self {
        let addr = SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::new(cfg.address.0, cfg.address.1, cfg.address.2, cfg.address.3),
            cfg.port,
        ));
        Self { svc, addr }
    }

    pub async fn serve(self) -> Result<()> {
        debug!(address = self.addr.to_string(), "now serving");
        tonic::transport::Server::builder()
            .accept_http1(true)
            .layer(GrpcWebLayer::new())
            .add_service(TodosServiceServer::new(self.svc))
            .serve(self.addr)
            .await?;

        Ok(())
    }
}
