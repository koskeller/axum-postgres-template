use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use sqlx::PgPool;
use tracing::log::{error, info};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ExamplePayload {
    ping: String,
}

#[tracing::instrument(name = "Example action", skip(pool))]
pub async fn example_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<ExamplePayload>,
) -> impl IntoResponse {
    info!("Handling example request");
    let result = sqlx::query!(
        "INSERT INTO example (uid, ping, created_at) VALUES ($1, $2, $3)",
        Uuid::new_v4(),
        payload.ping,
        Utc::now()
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!("failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
