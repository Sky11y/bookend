use crate::models::ModelController;
use axum::{Router, middleware::from_fn, routing::get};

use crate::error::log_app_errors;

use crate::web::routes_books::{get_book, list_books};

pub fn build_router(mc: ModelController) -> Router {
    Router::new()
        .route("/books", get(list_books))
        .route("/books/{id}", get(get_book))
        .layer(from_fn(log_app_errors))
        .with_state(mc)
}
