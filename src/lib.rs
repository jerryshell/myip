pub async fn ip_service(
    axum::extract::ConnectInfo(client_ip): axum::extract::ConnectInfo<std::net::SocketAddr>,
    request_header_map: axum::http::HeaderMap,
) -> (axum::http::StatusCode, axum_extra::response::ErasedJson) {
    tracing::info!("{:?}", request_header_map);
    let header_value_iter = request_header_map.get_all("x-forwarded-for").iter();
    let client_ip = match header_value_iter.last() {
        None => client_ip.ip().to_string(),
        Some(client_ip) => client_ip.to_str().unwrap().to_string(),
    };
    match ip(client_ip).await {
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
pub async fn ip(client_ip: String) -> Result<serde_json::Map<String, serde_json::Value>, String> {
    // match Locator::get(&client_ip, Service::IpApi).await {
    // Ok(ip) => {
    let result = serde_json::json!({
     "ip": client_ip,
    //  "latitude": ip.latitude,
    //  "longitude": ip.longitude,
    //  "city": ip.city,
    //  "region": ip.region,
    //  "country": ip.country,
    //  "timezone": ip.timezone,
     "repository": "https://github.com/jerryshell/myip",
     "license": "https://choosealicense.com/licenses/agpl-3.0",
    });
    Ok(result.as_object().unwrap().to_owned())
    // }
    // Err(e) => {
    //     tracing::warn!("{}", e);
    //     let result = json!({
    //      "ip": client_ip,
    //      "repository": "https://github.com/jerryshell/myip",
    //      "license": "https://choosealicense.com/licenses/agpl-3.0",
    //     });
    //     Ok(result.as_object().unwrap().to_owned())
    // }
    // }
}
