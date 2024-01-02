use axum::Json;
use serde_json::{json, Value};

pub async fn health_check_handler() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
