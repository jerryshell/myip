use anyhow::Result;
use axum::http::StatusCode;
use axum_client_ip::ClientIp;
use axum_extra::response::ErasedJson;
use cached::proc_macro::once;
use ipgeolocate::{Locator, Service};
use serde_json::{json, Map, Value};
use tracing::instrument;

#[instrument]
pub async fn ip_service(ClientIp(client_ip): ClientIp) -> (StatusCode, ErasedJson) {
    match ip(client_ip.to_string()).await {
        Ok(ip_info_map) => (StatusCode::OK, ErasedJson::pretty(ip_info_map)),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            ErasedJson::pretty(json! ({
                "error": e.to_string(),
                "repository": "https://github.com/jerryshell/myip",
                "license": "https://choosealicense.com/licenses/agpl-3.0",
            })),
        ),
    }
}

#[instrument]
#[once(time = 60, result = true, sync_writes = true)]
pub async fn ip(client_ip: String) -> Result<Map<String, Value>> {
    match Locator::get(&client_ip, Service::IpApi).await {
        Ok(ip) => {
            let result = json!({
             "ip": ip.ip,
             "latitude": ip.latitude,
             "longitude": ip.longitude,
             "city": ip.city,
             "region": ip.region,
             "country": ip.country,
             "timezone": ip.timezone,
             "repository": "https://github.com/jerryshell/myip",
             "license": "https://choosealicense.com/licenses/agpl-3.0",
            });
            Ok(result.as_object().unwrap().to_owned())
        }
        Err(e) => Err(e.into()),
    }
}
