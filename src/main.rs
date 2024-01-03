#[tokio::main]
async fn main() {
    // init env
    dotenv::dotenv().ok();

    // init ipinfo
    let ipinfo = tokio::task::spawn_blocking(move || {
        let ipinfo_config = ipinfo::IpInfoConfig {
            token: Some(std::env::var("IPINFO_TOKEN").unwrap()),
            ..Default::default()
        };
        ipinfo::IpInfo::new(ipinfo_config).expect("should construct")
    })
    .await
    .unwrap();

    // init tracing
    let file_appender = tracing_appender::rolling::daily("./", "myip.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();

    // cors
    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // init route
    let app = axum::Router::new()
        .route("/", axum::routing::get(myip::ip_service))
        .layer(cors)
        .layer(axum::Extension(std::sync::Arc::from(
            std::sync::Mutex::from(ipinfo),
        )));

    // init ip addr
    let ip_addr = std::env::var("IP_ADDR")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)));

    // init port
    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4000);
    tracing::info!("port {}", port);

    // init socket addr
    let socket_addr = std::net::SocketAddr::new(ip_addr, port);
    tracing::info!("socket_addr {}", socket_addr);

    // run app
    let app = app.into_make_service_with_connect_info::<std::net::SocketAddr>();
    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("bind failed");
    axum::serve(listener, app).await.expect("serve failed");
}
