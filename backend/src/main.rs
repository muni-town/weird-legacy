use axum::{extract::State, http::StatusCode, routing::get, Router};
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    Row,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let postgres_user = std::env::var("POSTGRES_USER").unwrap_or("postgres".to_string());
    let postgres_pass = std::env::var("POSTGRES_PASSWORD").unwrap_or("pass".to_string());
    let postgres_db = std::env::var("POSTGRES_DB").unwrap_or("db".to_string());
    let db_connection_str =
        format!("postgres://{postgres_user}:{postgres_pass}@localhost:5432/{postgres_db}");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/", get(using_connection_pool_extractor))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    let rows = sqlx::query("select * from page")
        .fetch_all(&pool)
        .await
        .map_err(internal_error);
    let rows = rows.unwrap();
    let str_result = rows
        .iter()
        .map(|r| {
            format!(
                "{} - {}",
                r.get::<String, _>("github_username"),
                r.get::<String, _>("display_name")
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    Ok(str_result)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
