use bookend::{
    models::{Book, ModelController},
    router::build_router,
};

use axum::{
    Router,
    body::Body,
    http::{self, Request, Response},
};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt; // for `oneshot`

#[allow(unused)]
pub enum Method {
    Get,
    Post(Value),
    Put,
    Delete,
}

pub fn spawn_app(pool: PgPool) -> Router {
    build_router(ModelController::new(pool))
}

pub async fn response(app: Router, uri: &str, method: Method) -> Response<Body> {
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

pub fn create_mock_payload() -> Value {
    json!({
        "isbn": "1234567890",
        "title": "Test",
        "description": "Testcription",
        "published": 2026
    })
}

pub async fn seed_mock_book(pool: PgPool) -> Value {
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

pub async fn get_json_body(response: Response<Body>) -> Value {
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body: Book = serde_json::from_slice(&body).unwrap();
    serde_json::to_value(&body).unwrap()
}
