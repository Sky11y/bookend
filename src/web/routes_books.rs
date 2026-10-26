use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use tracing::instrument;

use crate::error::Result;
use crate::models::{Book, BookForCreate, ModelController};

pub fn routes_books() -> Router<ModelController> {
    Router::new()
        .route("/", get(list_books).post(create_book))
        .route("/{id}", get(get_book).delete(delete_book))
}

#[instrument(skip(mc))]
async fn list_books(State(mc): State<ModelController>) -> Result<Json<Vec<Book>>> {
    let books = mc.list_books().await?;
    Ok(Json(books))
}

#[instrument(skip(mc))]
async fn get_book(State(mc): State<ModelController>, Path(id): Path<i64>) -> Result<Json<Book>> {
    let book = mc.get_book(id).await?;
    Ok(Json(book))
}

#[instrument(skip(mc))]
async fn create_book(
    State(mc): State<ModelController>,
    Json(book): Json<BookForCreate>,
) -> Result<(StatusCode, Json<Book>)> {
    let book = mc.create_book(book).await?;
    Ok((StatusCode::CREATED, Json(book)))
}

#[instrument(skip(mc))]
async fn delete_book(State(mc): State<ModelController>, Path(id): Path<i64>) -> Result<()> {
    mc.delete_book(id).await?;
    Ok(())
}
