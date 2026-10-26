use axum::{
    Json, Router,
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use serde_json::json;
use tower_http::trace::TraceLayer;

use crate::models::ModelController;
use crate::web::routes_books::routes_books;

pub fn build_router(mc: ModelController) -> Router {
    let book_routes = routes_books();

    Router::new()
        .nest("/books", book_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(mc)
        .fallback(fallback)
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({"error": format!("Not found {uri}")})),
    )
}
