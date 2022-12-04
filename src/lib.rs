use axum::http::StatusCode;
use axum::Json;
use axum_client_ip::ClientIp;
use serde_json::{json, Value};
use tracing::{info, instrument};

#[instrument]
pub async fn ip(ClientIp(client_ip): ClientIp) -> (StatusCode, Json<Value>) {
    let result = json!({
     "ip": client_ip,
     "sourceCode": "https://github.com/jerryshell/myip",
     "license": "https://choosealicense.com/licenses/agpl-3.0",
    });
    info!("result={}", result);
    (StatusCode::OK, Json(result))
}
