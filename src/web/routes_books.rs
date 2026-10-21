use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};

use crate::error::Result;
use crate::models::{Book, BookForCreate, ModelController};

pub fn routes_books() -> Router<ModelController> {
    Router::new()
        .route("/", get(list_books).post(create_book))
        .route("/{id}", get(get_book).delete(delete_book))
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

async fn create_book(
    State(mc): State<ModelController>,
    Json(book): Json<BookForCreate>,
) -> Result<Response> {
    println!("  >>> {:<10} - create_book", "HANDLER");

    // TODO: Book creation return id. Do I want to do anything with it?
    let _book = mc.create_book(book).await?;
    Ok((StatusCode::CREATED).into_response())
}

async fn delete_book(State(mc): State<ModelController>, Path(id): Path<i64>) -> Result<()> {
    println!("  >>> {:<10} - delete_book", "HANDLER");

    mc.delete_book(id).await?;
    Ok(())
}
