use rust_grpc_server_demo::server;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    server::run().await?;
    Ok(())
}
