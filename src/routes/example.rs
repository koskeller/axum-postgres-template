use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use tracing::log::{error, info};
use uuid::Uuid;

use crate::AppState;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ExamplePayload {
    ping: String,
}

#[tracing::instrument(name = "Example action", skip(state))]
pub async fn example_handler(
    State(state): State<AppState>,
    Json(payload): Json<ExamplePayload>,
) -> impl IntoResponse {
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
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!("failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
