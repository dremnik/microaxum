use axum::Router;
use axum::middleware;
use axum::routing::get;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use hyper::StatusCode;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

// - crate -
use crate::api::users::route as users;
use crate::auth::inject_user_context;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

pub fn build_public_router() -> Router {
    Router::new().route("/health", get(health_check))
}

pub fn build_api_router(app_state: AppState) -> Router {
    Router::new()
        .nest("/users", users::router())
        .with_state(app_state.clone())
        .layer(middleware::from_fn(inject_user_context))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
}

async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}
