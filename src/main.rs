#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let app = axum::Router::new()
        .route("/", axum::routing::get(myip::ip_service))
        .layer(cors);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4000);
    tracing::info!("port={}", port);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("addr={}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .expect("axum::Server::bind().serve() err");
}
