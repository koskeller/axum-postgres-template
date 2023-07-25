use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::log::{error, info};
use uuid::Uuid;

use crate::{errors::HTTPError, AppState};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamplePayload {
    ping: String,
}

#[tracing::instrument(name = "Example action", skip(state))]
pub async fn example_handler(
    State(state): State<AppState>,
    Json(payload): Json<ExamplePayload>,
) -> Result<StatusCode, HTTPError> {
    info!("Handling example request");
    let result = sqlx::query!(
        "INSERT INTO example (uid, ping, created_at) VALUES ($1, $2, $3)",
        Uuid::new_v4(),
        payload.ping,
        Utc::now()
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            error!("failed to execute query: {}", e);
            Err(HTTPError::new("Example error").with_status(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}
