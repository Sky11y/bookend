#![allow(unused)]
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware::from_fn,
    routing::get,
};
use serde_json::{Value, json};
use sqlx::PgPool;

mod error;
mod models;

use crate::error::{Error, Result, log_app_errors};
use models::Book;

async fn get_books(State(pool): State<PgPool>) -> Result<Json<Vec<Book>>> {
    println!("  >>> {:<10} - get_books", "HANDLER");

    let books = sqlx::query_as::<_, Book>("SELECT * FROM books ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(books))
}

async fn hello_world() -> Result<&'static str> {
    println!("  >>> {:<10} - hello_world", "HANDLER");

    Ok("Hello World!")
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connext to database");

    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations");
    //
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/books", get(get_books))
        .layer(from_fn(log_app_errors))
        .with_state(pool);

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Listener");

    println!(
        "Server listening on port {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.expect("Could not serve");
}
