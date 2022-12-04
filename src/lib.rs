use axum::http::StatusCode;
use axum::Json;
use axum_client_ip::ClientIp;
use serde_json::{json, Value};
use tracing::info;

pub async fn ip(ClientIp(client_ip): ClientIp) -> (StatusCode, Json<Value>) {
    info!("client_ip {}", client_ip);
    (
        StatusCode::OK,
        Json(json!({
         "ip": client_ip,
         "sourceCode": "https://github.com/jerryshell/myip",
         "license": "https://choosealicense.com/licenses/agpl-3.0",
        })),
    )
}
