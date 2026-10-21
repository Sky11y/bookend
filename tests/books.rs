#![allow(unused)]
use bookend::{
    models::{Book, BookForCreate, ModelController},
    router::build_router,
};

use axum::{
    Router,
    body::Body,
    http::{self, Request, Response, StatusCode},
};
use serde::Serialize;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt; // for `oneshot`

enum Method {
    Get,
    Post(Value),
    Put,
    Delete,
}

fn spawn_app(pool: PgPool) -> Router {
    build_router(ModelController::new(pool))
}

async fn response(app: Router, uri: &str, method: Method) -> Response<Body> {
    match method {
        Method::Get => app
            .oneshot(Request::get(uri).body(Body::empty()).unwrap())
            .await
            .unwrap(),
        Method::Post(payload) => {
            let json = serde_json::to_string(&payload).expect("serialize payload");
            app.oneshot(
                Request::post(uri)
                    .header(http::header::CONTENT_TYPE, "Application/json")
                    .body(Body::from(json))
                    .unwrap(),
            )
            .await
            .unwrap()
        }
        Method::Put => app
            .oneshot(Request::put(uri).body(Body::empty()).unwrap())
            .await
            .unwrap(),
        Method::Delete => app
            .oneshot(Request::delete(uri).body(Body::empty()).unwrap())
            .await
            .unwrap(),
    }
}

fn create_mock_payload() -> Value {
    json!({
        "isbn": "1234567890",
        "title": "Test",
        "description": "Testcription",
        "published": 2026
    })
}

async fn seed_mock_book(pool: PgPool) -> Value {
    let id = sqlx::query_scalar!(
        "INSERT INTO books (isbn, title, description, published) VALUES ($1, $2, $3, $4) RETURNING id",
        "1234567890",
        "Test",
        "Testcription",
        2026
        )
        .fetch_one(&pool)
        .await
        .unwrap();

    json!({
            "id": id,
            "isbn": "1234567890",
            "title": "Test",
            "description": "Testcription",
            "published": 2026
    })
}

#[sqlx::test(migrations = "tests/migrations")]
async fn books_get_all_ok_empty_database(pool: PgPool) {
    let app = spawn_app(pool);

    let response = response(app, "/books", Method::Get).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: Vec<Book> = serde_json::from_slice(&body).unwrap();
    assert_eq!(body.len(), 0);
}

#[sqlx::test]
async fn books_get_ok_all_books(pool: PgPool) {
    let app = spawn_app(pool);

    let response = response(app, "/books", Method::Get).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: Vec<Book> = serde_json::from_slice(&body).unwrap();
    assert!(!body.is_empty());
}

#[sqlx::test(migrations = "tests/migrations")]
async fn books_get_id_ok_seeded_book(pool: PgPool) {
    let app = spawn_app(pool.clone());
    let payload = seed_mock_book(pool.clone()).await;

    let uri = format!("/books/{}", payload["id"]);
    let response = response(app, &uri, Method::Get).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: Book = serde_json::from_slice(&body).unwrap();
    let body = serde_json::to_value(&body).unwrap();
    assert_eq!(
        body,
        json!({
            "id": payload["id"],
            "isbn": "1234567890",
            "title": "Test",
            "description": "Testcription",
            "published": 2026
        })
    );
}

#[sqlx::test(migrations = "tests/migrations")]
async fn books_get_invalid_id_err_404(pool: PgPool) {
    let app = spawn_app(pool);

    let response = response(app, "/books/9999", Method::Get).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn books_post_ok(pool: PgPool) {
    let app = spawn_app(pool.clone());
    let payload = create_mock_payload();

    let response = response(app, "/books", Method::Post(payload)).await;
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[sqlx::test(migrations = "tests/migrations")]
async fn books_delete_ok(pool: PgPool) {
    let app = spawn_app(pool.clone());
    let payload = seed_mock_book(pool.clone()).await;

    let uri = format!("/books/{}", payload["id"]);
    let response = response(app, &uri, Method::Delete).await;
    assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test(migrations = "tests/migrations")]
async fn books_delete_err_404(pool: PgPool) {
    let app = spawn_app(pool.clone());

    let response = response(app, "/books/9999", Method::Delete).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
