use sqlx::PgPool;

use bookend::error::Result;
use bookend::models::ModelController;
use bookend::router::build_router;

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
    let app = build_router(mc);

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Listener");

    println!(
        "Server listening on port {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.expect("Could not serve");

    Ok(())
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_root() {
        assert_eq!(true, true);
    }
}
