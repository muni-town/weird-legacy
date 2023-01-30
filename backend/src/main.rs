use axum::{
    extract::{Path, State},
    http::{StatusCode, HeaderValue, Method},
    response::IntoResponse,
    routing::get,
    Router,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use common::Link;

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

    let db_connection_str = format!(
        "postgres://{}:{}@localhost:5432/{}",
        std::env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string()),
        std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "pass".to_string()),
        std::env::var("POSTGRES_DB").unwrap_or_else(|_| "db".to_string())
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/:github_username", get(get_page))
        .layer(CorsLayer::new()
               .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
               .allow_methods([Method::GET]))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_page(
    Path(github_username): Path<String>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let page = sqlx::query_as::<_, Link>("select * from links where github_username = $1")
        .bind(github_username)
        .fetch_all(&pool)
        .await
        .map_err(internal_error);
    (StatusCode::OK, axum::Json(page.unwrap()))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
