use anyhow::Result;
use resources::grpc::{config::Config, server::Server, service::Service};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::new((0, 0, 0, 0), 50051);
    let svc = Service::new();
    let srv = Server::new(svc, cfg);

    srv.serve().await?;

    Ok(())
}
