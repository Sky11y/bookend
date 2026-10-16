use axum::{
    Json, Router,
    http::{StatusCode, Uri},
    middleware::from_fn,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::error::log_app_errors;
use crate::models::ModelController;
use crate::web::routes_books::routes_books;

pub fn build_router(mc: ModelController) -> Router {
    let book_routes = routes_books();

    Router::new()
        .nest("/books", book_routes)
        .layer(from_fn(log_app_errors))
        .with_state(mc)
        .fallback(fallback)
}

async fn fallback(uri: Uri) -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(json!({"error": format!("Not found {uri}")})),
    )
        .into_response()
}
