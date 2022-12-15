use axum::{routing::get, Router};
use myip::*;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, instrument};

#[instrument]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new().route("/", get(ip_service)).layer(cors);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4000);
    info!("port={}", port);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("addr={}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("axum::Server::bind().serve() err");
}
