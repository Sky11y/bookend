use axum::Json;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::sync::Arc;

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

        // this is the error we send back to client
        let (status, error, err) = match &self {
            Error::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_owned(), None),
            Error::Database(_err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
                Some(self),
            ),
        };

        let mut response = (status, Json(ErrorResponse { error })).into_response();
        // this is the error we print to console (only print if server error)
        if let Some(err) = err {
            response.extensions_mut().insert(Arc::new(err));
        }
        response
    }
}

pub async fn log_app_errors(request: Request, next: Next) -> Response {
    let response = next.run(request).await;

    if let Some(err) = response.extensions().get::<Arc<Error>>() {
        eprintln!("  >>> {:<10} - {err}", "ERROR");
    }
    response
}
