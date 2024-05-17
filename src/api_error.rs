use axum::{
    extract::rejection::JsonRejection,
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Invalid request.")]
    RequestValidation(#[from] JsonRejection),

    #[error("A database error occurred.")]
    Database(#[from] sqlx::Error),

    #[error("An internal server error occurred.")]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize, Deserialize)]
pub struct ApiErrorResp {
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // Log detailed error for telemetry.
        let error_to_log = match &self {
            ApiError::RequestValidation(ref err) => match err {
                JsonRejection::JsonDataError(e) => e.body_text(),
                JsonRejection::JsonSyntaxError(e) => e.body_text(),
                JsonRejection::MissingJsonContentType(_) => {
                    "Missing `Content-Type: application/json` header".to_string()
                }
                JsonRejection::BytesRejection(_) => "Failed to buffer request body".to_string(),
                _ => "Unknown error".to_string(),
            },
            ApiError::Database(ref err) => format!("{}", err),
            ApiError::Internal(ref err) => format!("{}", err),
        };
        error!("{}", error_to_log);

        // Create a general response to hide specific implementation details.
        let resp = ApiErrorResp {
            message: self.to_string(),
        };

        // Determine the appropriate status code.
        let status = match self {
            ApiError::RequestValidation(_) => StatusCode::BAD_REQUEST,
            ApiError::Database(_) | ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(resp)).into_response()
    }
}
