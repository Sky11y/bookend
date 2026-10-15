#![allow(unused)]
use bookend::{
    models::{Book, ModelController},
    router::build_router,
};

use axum::{
    Router,
    body::Body,
    http::{self, Request, Response, StatusCode},
};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt; // for `oneshot`

fn spawn_app(pool: PgPool) -> Router {
    build_router(ModelController::new(pool))
}

async fn get_response(app: Router, uri: &str) -> Response<Body> {
    app.oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap()
}

async fn post_response<T: Serialize>(app: Router, uri: String, payload: &T) -> Response<Body> {
    let json = serde_json::to_string(payload).expect("serialize payload");
    app.oneshot(
        Request::post(uri)
            .header(http::header::CONTENT_TYPE, "Application/json")
            .body(Body::from(json))
            .unwrap(),
    )
    .await
    .unwrap()
}

#[sqlx::test(migrations = "tests/migrations")]
async fn get_books_on_empty_database_ok(pool: PgPool) {
    let app = spawn_app(pool);

    let response = get_response(app, "/books").await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: Vec<Book> = serde_json::from_slice(&body).unwrap();
    assert_eq!(body.len(), 0);
}

#[sqlx::test]
async fn get_books_ok_all_books(pool: PgPool) {
    let app = spawn_app(pool);

    let response = get_response(app, "/books").await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: Vec<Book> = serde_json::from_slice(&body).unwrap();
    assert!(!body.is_empty());
}

#[sqlx::test(migrations = "tests/migrations")]
async fn get_books_id_ok_seeded_book(pool: PgPool) {
    let app = spawn_app(pool.clone());
    let id = sqlx::query_scalar!(
        r#"INSERT INTO books (isbn, title, description, published) VALUES ($1, $2, $3, $4) RETURNING id"#,
        "1234567890",
        "Test",
        "Testcription",
        2026
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let uri = format!("/books/{id}");
    let response = get_response(app, &uri).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: Book = serde_json::from_slice(&body).unwrap();
    let body = serde_json::to_value(&body).unwrap();
    assert_eq!(
        body,
        json!({
            "id": id,
            "isbn": "1234567890",
            "title": "Test",
            "description": "Testcription",
            "published": 2026
        })
    );
}

#[sqlx::test(migrations = "tests/migrations")]
async fn get_books_invalid_id_err_404(pool: PgPool) {
    let app = spawn_app(pool);

    let response = get_response(app, "/books/9999").await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
