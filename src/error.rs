use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

// can use Result<T> instead of Result<T, E> on return types
pub type Result<T> = core::result::Result<T, Error>;

// for service provider errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("resource not found")]
    NotFound,
    #[error("database error")]
    Database(#[from] sqlx::Error),
    // LoginFail,
    // AuthFail,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            error: String,
        }

        let (status, error) = match &self {
            Error::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
            Error::Database(err) => {
                // print database error to console
                tracing::error!(error = ?err, "database error: ");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
        };

        (
            status,
            Json(ErrorResponse {
                error: error.to_owned(),
            }),
        )
            .into_response()
    }
}
