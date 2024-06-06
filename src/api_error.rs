use axum::{
    extract::rejection::JsonRejection,
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

/// Custom error type for the API.
/// The `#[from]` attribute allows for easy conversion from other error types.
#[derive(Error, Debug)]
pub enum ApiError {
    /// Converts from an Axum built-in extractor error.
    #[error("Invalid payload.")]
    InvalidJsonBody(#[from] JsonRejection),

    /// For errors that occur during manual validation.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Converts from `sqlx::Error`.
    #[error("A database error has occurred.")]
    DatabaseError(#[from] sqlx::Error),

    /// Converts from any `anyhow::Error`.
    #[error("An internal server error has occurred.")]
    InternalError(#[from] anyhow::Error),
}

#[derive(Serialize, Deserialize)]
pub struct ApiErrorResp {
    pub message: String,
}

// The IntoResponse implementation for ApiError logs the error message.
//
// To avoid exposing implementation details to API consumers, we separate
// the message that we log from the API response message.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // Log detailed error for telemetry.
        let error_to_log = match &self {
            ApiError::InvalidJsonBody(ref err) => match err {
                JsonRejection::JsonDataError(e) => e.body_text(),
                JsonRejection::JsonSyntaxError(e) => e.body_text(),
                JsonRejection::MissingJsonContentType(_) => {
                    "Missing `Content-Type: application/json` header".to_string()
                }
                JsonRejection::BytesRejection(_) => "Failed to buffer request body".to_string(),
                _ => "Unknown error".to_string(),
            },
            ApiError::InvalidRequest(_) => format!("{}", self),
            ApiError::DatabaseError(ref err) => format!("{}", err),
            ApiError::InternalError(ref err) => format!("{}", err),
        };
        error!("{}", error_to_log);

        // Create a generic response to hide specific implementation details.
        let resp = ApiErrorResp {
            message: self.to_string(),
        };

        // Determine the appropriate status code.
        let status = match self {
            ApiError::InvalidJsonBody(_) | ApiError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::DatabaseError(_) | ApiError::InternalError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status, Json(resp)).into_response()
    }
}
