use bookend::models::Book;

use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;

mod test_helpers;
use test_helpers::{
    Method, create_mock_payload, get_json_body, response, seed_mock_book, spawn_app,
};

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

    let body = get_json_body(response).await;
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

    let body = get_json_body(response).await;
    assert_eq!(
        body,
        json!({
            "id": 3,
            "isbn": "1234567890",
            "title": "Test",
            "description": "Testcription",
            "published": 2026
        })
    );
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
