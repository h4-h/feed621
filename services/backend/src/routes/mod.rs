use std::{sync::Arc, time::Duration};
use aide::axum::ApiRouter;
use axum::{http::header::AUTHORIZATION, response::IntoResponse};
use tower_http::{catch_panic::CatchPanicLayer, compression::CompressionLayer, cors::{self, CorsLayer}, sensitive_headers::SetSensitiveHeadersLayer, timeout::TimeoutLayer};

use crate::error::AppError;

mod openapi;
mod test_routes;
mod user_routes;
mod topic_routes;

pub(crate) fn configure_routes() -> axum::Router {
    let mut api = openapi::generate_api();
    
    aide::axum::ApiRouter::new()
        .fallback(fallback)
        .nest("/docs", openapi::docs_routes())
        .nest("/api", api_routes())
        .finish_api_with(&mut api, openapi::configure_api)
        .layer(axum::Extension(Arc::new(api)))
        .layer((
            TimeoutLayer::new(Duration::from_secs(30)),
            CorsLayer::new().allow_origin(cors::Any),
            CompressionLayer::new(),
            CatchPanicLayer::new(),
            SetSensitiveHeadersLayer::new([AUTHORIZATION]),
        ))
}

fn api_routes() -> ApiRouter {
    ApiRouter::new()
        .merge(test_routes::test_routes())
        .merge(user_routes::user_routes())
        .merge(topic_routes::topic_routes())
}

async fn fallback() -> impl axum::response::IntoResponse {
    AppError::NotFound.into_response()
}
