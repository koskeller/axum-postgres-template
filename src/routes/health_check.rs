use axum::Json;
use serde_json::{json, Value};

use crate::api_error::ApiError;

pub async fn health_check() -> Result<Json<Value>, ApiError> {
    Ok(Json(json!({ "status": "ok" })))
}
