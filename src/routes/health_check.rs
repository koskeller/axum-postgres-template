use axum::Json;
use serde_json::{json, Value};

use crate::errors::HTTPError;

pub async fn health_check_handler() -> Result<Json<Value>, HTTPError> {
    Ok(Json(json!({ "status": "ok" })))
}
