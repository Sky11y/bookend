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

use bookend::error::{Error, Result, log_app_errors};
use bookend::models::{Book, ModelController};
use bookend::router;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connext to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let mc = ModelController::new(pool);
    let app = router::build_router(mc);

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Listener");

    println!(
        "Server listening on port {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.expect("Could not serve");

    Ok(())
}

async fn hello_world() -> Result<&'static str> {
    println!("  >>> {:<10} - hello_world", "HANDLER");

    Ok("Hello World!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_root() {
        let response = hello_world().await.unwrap();
        assert_eq!(response, "Hello World!");
    }
}
