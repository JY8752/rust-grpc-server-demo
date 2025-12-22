use std::error::Error;
use tokio::{signal, sync::mpsc};
use tonic::{include_file_descriptor_set, transport::Server};
use tonic_reflection::server::Builder;
use tonic_web::GrpcWebLayer;

use crate::user;
use crate::weather;

const FILE_DESCRIPTOR_SET: &[u8] = include_file_descriptor_set!("tonic_descriptor");

pub async fn run() -> Result<(), Box<dyn Error>> {
    let reflection_service_v1 = Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let reflection_service_v1alpha = Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    let addr = "127.0.0.1:50051".parse()?;

    println!("Server listening on {addr}");

    let (_shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);
    let shutdown_signal = shutdown_signal(shutdown_rx);

    Server::builder()
        .accept_http1(true)
        // .layer(tower_http::cors::CorsLayer::new())
        .layer(GrpcWebLayer::new())
        .add_service(reflection_service_v1)
        .add_service(reflection_service_v1alpha)
        .add_service(user::get_service())
        .add_service(weather::get_service())
        // .serve(addr)
        .serve_with_shutdown(addr, shutdown_signal)
        .await?;

    Ok(())
}

// ref: https://github.com/hyperium/tonic/issues/1820
async fn shutdown_signal(mut shutdown_on_err: mpsc::Receiver<()>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install SIGINT handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    let shutdown_on_err_future = async {
        shutdown_on_err.recv().await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
        _ = shutdown_on_err_future => {},
    }
    println!("shutdown signal received");
}
