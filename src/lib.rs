pub async fn ip_service(
    axum::Extension(ipinfo_arc): axum::Extension<std::sync::Arc<std::sync::Mutex<ipinfo::IpInfo>>>,
    axum::extract::ConnectInfo(connect_info): axum::extract::ConnectInfo<std::net::SocketAddr>,
    request_header_map: axum::http::HeaderMap,
) -> (axum::http::StatusCode, axum_extra::response::ErasedJson) {
    tracing::info!("request_header_map {:?}", request_header_map);

    let header_value_iter = request_header_map.get_all("x-forwarded-for").iter();
    let client_ip = match header_value_iter.last() {
        None => connect_info.ip().to_string(),
        Some(header_value) => match header_value.to_str() {
            Ok(header_value_str) => header_value_str.to_string(),
            Err(_) => connect_info.ip().to_string(),
        },
    };
    tracing::info!("client_ip {client_ip}");

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

pub async fn get_ip_info(
    ipinfo_arc: std::sync::Arc<std::sync::Mutex<ipinfo::IpInfo>>,
    client_ip: &str,
) -> Result<serde_json::Map<String, serde_json::Value>, String> {
    let client_ip_for_spawn = client_ip.to_string();
    let result = tokio::task::spawn_blocking(move || {
        ipinfo_arc.lock().unwrap().lookup(&[&client_ip_for_spawn])
    })
    .await
    .unwrap();

    match result {
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

#[cfg(test)]
mod tests {
    mod get_ip_info {
        #[tokio::test]
        async fn test() -> Result<(), Box<dyn std::error::Error>> {
            dotenv::dotenv().ok();

            let ipinfo_config = ipinfo::IpInfoConfig {
                token: Some(std::env::var("IPINFO_TOKEN").unwrap()),
                ..Default::default()
            };
            let ipinfo = tokio::task::spawn_blocking(|| {
                ipinfo::IpInfo::new(ipinfo_config).expect("should construct")
            })
            .await
            .unwrap();
            let ipinfo_arc = std::sync::Arc::from(std::sync::Mutex::from(ipinfo));

            let result = crate::get_ip_info(ipinfo_arc, "8.8.8.8").await.unwrap();
            assert_eq!(
                result
                    .get("detail")
                    .unwrap()
                    .get("hostname")
                    .unwrap()
                    .to_owned(),
                "dns.google".to_owned()
            );

            Ok(())
        }
    }
}
