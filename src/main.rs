#![allow(unused)]
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde_json::{Value, json};
use sqlx::PgPool;

mod models;

use models::Book;

async fn get_books(pool: &PgPool) -> Result<Vec<Book>, sqlx::Error> {
    sqlx::query_as::<_, Book>("SELECT * FROM books ORDER BY id")
        .fetch_all(pool)
        .await
}

async fn books(State(pool): State<PgPool>) -> Result<Json<Vec<Book>>, StatusCode> {
    get_books(&pool).await.map(Json).map_err(|e| {
        println!("{:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
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
    let app = Router::new().route("/books", get(books)).with_state(pool);

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Listener");

    println!("Server listening port 8080");
    axum::serve(listener, app).await.expect("Could not serve");
}
