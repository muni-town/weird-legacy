use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use common::Link;

const DB_URL: &str = "sqlite:weird.db";

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

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let conn = SqlitePool::connect(DB_URL).await.unwrap();

    let _create_table = sqlx::query(
        r#"
    CREATE TABLE links (
        url TEXT PRIMARY KEY NOT NULL,
        title TEXT NOT NULL,
        github_username TEXT NOT NULL
    );
    "#,
    )
    .execute(&conn)
    .await;

    let app = Router::new()
        .route("/:github_username", get(get_page))
        .route("/create", post(create_page))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(Any),
        )
        .with_state(conn);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_page(
    Path(github_username): Path<String>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    let page = sqlx::query_as::<_, Link>("SELECT * FROM links WHERE github_username = ?")
        .bind(&github_username)
        .fetch_all(&pool)
        .await
        .map_err(internal_error);

    (StatusCode::OK, axum::Json(page.unwrap()))
}

#[axum_macros::debug_handler]
async fn create_page(
    State(pool): State<SqlitePool>,
    Json(links): Json<Vec<Link>>,
) -> impl IntoResponse {
    for link in links.iter() {
        _ = sqlx::query_as::<_, Link>(
            "INSERT INTO links (url, title, github_username)  VALUES (?1, ?2, ?3)",
        )
        .bind(&link.url)
        .bind(&link.title)
        .bind(&link.github_username)
        .fetch_one(&pool)
        .await
        .map_err(internal_error);
    }

    (
        StatusCode::CREATED,
        format!("Created page for {}", links[0].github_username),
    )
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
