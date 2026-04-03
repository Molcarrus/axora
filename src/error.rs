use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OracleError {
    #[error("Failed to fetch from source {source_name}: {message}")]
    SourceFetch {
        source_name: String,
        message: String,
    },

    #[error("Feed not found: {0}")]
    FeedNotFound(String),

    #[error("Invalid request : {0}")]
    BadRequest(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl IntoResponse for OracleError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_code, message) = match &self {
            OracleError::FeedNotFound(_) => {
                (StatusCode::NOT_FOUND, "FEED_NOT_FOUND", self.to_string())
            }
            OracleError::BadRequest(_) => {
                (StatusCode::BAD_REQUEST, "BAD_REQUEST", self.to_string())
            }
            OracleError::SourceFetch { .. } => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SOURCE_UNAVAILABLE",
                self.to_string(),
            ),
            _ => {
                tracing::error!(error = %self, "Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An internal error occured".to_string(),
                )
            }
        };

        let body = json!({
            "error": {
                "code": error_code,
                "message": message,
            }
        });

        (status, Json(body)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, OracleError>;
