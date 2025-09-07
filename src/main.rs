use std::env;
use std::net::SocketAddr;

use axum::Router;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::{AppState, build_api_router, build_public_router};

mod api;
mod app;
mod auth;
mod db;
mod error;
mod schema;

const MAX_DB_CONNECTIONS: usize = 10;

#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env.local").ok();

    tracing_subscriber::registry()
        .with(
            // tracing_subscriber::EnvFilter::try_from_default_env()
            //     .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ---- DB ----
    tracing::info!("Initializing database connection pool");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        tracing::error!("DATABASE_URL environment variable not set");
        panic!("DATABASE_URL must be set");
    });

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    let pool = match Pool::builder(config).max_size(MAX_DB_CONNECTIONS).build() {
        Ok(pool) => {
            tracing::info!("Database connection pool successfully created");
            pool
        }
        Err(e) => {
            tracing::error!("Failed to build database pool: {}", e);
            panic!("Failed to build database pool: {}", e);
        }
    };

    match pool.get().await {
        Ok(_) => tracing::info!("Successfully connected to database"),
        Err(e) => tracing::warn!("Initial connection test failed: {}", e),
    }
    // ---- END DB ---

    let app_state = AppState { db_pool: pool };

    let public_router = build_public_router();
    let api_router = build_api_router(app_state);
    let app = public_router.merge(Router::new().nest("/v1", api_router));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| {
            tracing::info!("PORT not set, defaulting to 8080");
            "8080".to_string()
        })
        .parse::<u16>()
        .unwrap_or_else(|e| {
            tracing::error!("Failed to parse PORT: {}", e);
            panic!("Failed to parse PORT: {}", e);
        });

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Failed to bind to address {}: {}", addr, e);
            panic!("Failed to bind to address: {}", e);
        });

    tracing::info!("Server started successfully");
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        tracing::error!("Server error: {}", e);
        panic!("Server error: {}", e);
    });
}
