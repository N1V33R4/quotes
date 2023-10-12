mod handlers;
use std::env;

use axum::routing::{get, Router, post, put};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env");
    let database_url = "postgres://postgres:123@localhost:5432/quotes?sslm";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/quotes", post(handlers::create_quote))
        .route("/quotes", get(handlers::read_quotes))
        .route("/quotes/:id", put(handlers::update_quote))
        .with_state(pool);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
