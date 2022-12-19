use axum::http::StatusCode;
use axum::Json;
use axum_client_ip::ClientIp;
use axum_extra::response::ErasedJson;
use ipgeolocate::{Locator, Service};
use serde_json::{json, Value};
use tracing::{info, instrument};

#[instrument]
pub async fn ip_service(ClientIp(client_ip): ClientIp) -> (StatusCode, ErasedJson) {
    let client_ip = client_ip.to_string();
    info!(client_ip);
    let (status_code, response_json) = ip(&client_ip).await;
    let response_map = response_json.as_object().unwrap();
    (status_code, ErasedJson::pretty(response_map))
}

#[instrument]
pub async fn ip(client_ip: &str) -> (StatusCode, Json<Value>) {
    let service = Service::IpApi;
    match Locator::get(client_ip, service).await {
        Ok(ip) => {
            let result = json!({
             "ip": ip.ip,
             "latitude": ip.latitude,
             "longitude": ip.longitude,
             "city": ip.city,
             "region": ip.region,
             "country": ip.country,
             "timezone": ip.timezone,
             "sourceCode": "https://github.com/jerryshell/myip",
             "license": "https://choosealicense.com/licenses/agpl-3.0",
            });
            info!("result={}", result);
            (StatusCode::OK, Json(result))
        }
        Err(error) => {
            let result = json!({
             "error": error.to_string(),
             "sourceCode": "https://github.com/jerryshell/myip",
             "license": "https://choosealicense.com/licenses/agpl-3.0",
            });
            info!("result={}", result);
            (StatusCode::BAD_REQUEST, Json(result))
        }
    }
}
