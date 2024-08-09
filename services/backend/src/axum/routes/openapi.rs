use std::sync::Arc;
use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use axum::{response::{Html, IntoResponse}, routing::get, Extension, Json};

const SCALAR_VERSION: &str = "1.24.73";

pub fn router() -> ApiRouter {
    ApiRouter::new()
        .route("/openapi.json", get(serve_json_spec))
        .route("/scalar", get(serve_scalar))
}

pub fn configure_info(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Feed621")
        .summary("Something that looks like REST API.")
        .description("REST API for reddit-like feed of e621 arts")
}

async fn serve_json_spec(
    Extension(api): Extension<Arc<OpenApi>>
) -> impl IntoResponse {
    Json(api)
}

async fn serve_scalar() -> impl IntoResponse {
    Html(format!(r#"
        <html>
          <head>
            <title>Feed621 documentation</title>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
          </head>
          <body>
            <script id="api-reference" data-url="/docs/openapi.json"></script>
            <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference@{}/dist/browser/standalone.min.js"></script>
          </body>
        </html>
    "#, SCALAR_VERSION))
}
