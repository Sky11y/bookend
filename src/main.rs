use sqlx::PgPool;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, registry, util::SubscriberInitExt};

use bookend::error::Result;
use bookend::models::ModelController;
use bookend::router::build_router;

#[tokio::main]
async fn main() -> Result<()> {
    // set up tracing
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
            "{}=debug,tower_http=warn,sqlx=warn",
            env!("CARGO_CRATE_NAME")
        )
        .into()
    });
    registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // set up database
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

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
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
