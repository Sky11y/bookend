use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};

use crate::error::Result;
use crate::models::{Book, ModelController};

pub fn routes_books() -> Router<ModelController> {
    Router::new()
        .route("/", get(list_books))
        .route("/{id}", get(get_book))
}

async fn list_books(State(mc): State<ModelController>) -> Result<Json<Vec<Book>>> {
    println!("  >>> {:<10} - list_books", "HANDLER");

    let books = mc.list_books().await?;
    Ok(Json(books))
}

async fn get_book(State(mc): State<ModelController>, Path(id): Path<i64>) -> Result<Json<Book>> {
    println!("  >>> {:<10} - get_book", "HANDLER");

    let book = mc.get_book(id).await?;
    Ok(Json(book))
}
