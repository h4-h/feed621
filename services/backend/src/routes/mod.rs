use std::time::Duration;
use axum::{http::header::AUTHORIZATION, response::IntoResponse};
use tower_http::{catch_panic::CatchPanicLayer, compression::CompressionLayer, cors::{self, CorsLayer}, sensitive_headers::SetSensitiveHeadersLayer, timeout::TimeoutLayer};
use crate::error::AppError;

mod user_routes;
mod topic_routes;
mod subscription_routes;

pub(crate) fn configure_routes() -> axum::Router {
    axum::Router::new()
        .fallback(fallback)
        .nest("/api", api_routes())
        .layer((
            TimeoutLayer::new(Duration::from_secs(30)),
            CorsLayer::new().allow_origin(cors::Any),
            CompressionLayer::new(),
            CatchPanicLayer::new(),
            SetSensitiveHeadersLayer::new([AUTHORIZATION]),
        ))
}

fn api_routes() -> axum::Router {
    axum::Router::new()
        .merge(user_routes::user_routes())
        .merge(topic_routes::topic_routes())
        .merge(subscription_routes::subscription_routes())
}

async fn fallback() -> impl axum::response::IntoResponse {
    AppError::NotFound.into_response()
}
