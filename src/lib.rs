pub async fn ip_service(
    axum::Extension(ipinfo_arc): axum::Extension<std::sync::Arc<std::sync::Mutex<ipinfo::IpInfo>>>,
    axum::extract::ConnectInfo(client_ip): axum::extract::ConnectInfo<std::net::SocketAddr>,
    request_header_map: axum::http::HeaderMap,
) -> (axum::http::StatusCode, axum_extra::response::ErasedJson) {
    tracing::info!("{:?}", request_header_map);
    let header_value_iter = request_header_map.get_all("x-forwarded-for").iter();
    let client_ip = match header_value_iter.last() {
        None => client_ip.ip().to_string(),
        Some(client_ip) => client_ip.to_str().unwrap().to_string(),
    };
    match get_ip_info(ipinfo_arc, &client_ip).await {
        Ok(ip_info_map) => (
            axum::http::StatusCode::OK,
            axum_extra::response::ErasedJson::pretty(ip_info_map),
        ),
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            axum_extra::response::ErasedJson::pretty(serde_json::json! ({
                "error": e,
                "repository": "https://github.com/jerryshell/myip",
                "license": "https://choosealicense.com/licenses/agpl-3.0",
            })),
        ),
    }
}

#[cached::proc_macro::once(time = 60, result = true)]
pub async fn get_ip_info(
    ipinfo_arc: std::sync::Arc<std::sync::Mutex<ipinfo::IpInfo>>,
    client_ip: &str,
) -> Result<serde_json::Map<String, serde_json::Value>, String> {
    match ipinfo_arc.lock().unwrap().lookup(&[client_ip]) {
        Err(e) => Err(e.to_string()),
        Ok(result) => {
            let result = serde_json::json!({
             "ip": client_ip,
             "detail": &result[client_ip],
             "repository": "https://github.com/jerryshell/myip",
             "license": "https://choosealicense.com/licenses/agpl-3.0",
            });
            Ok(result.as_object().unwrap().to_owned())
        }
    }
}
