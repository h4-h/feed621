use std::sync::Arc;
use aide::{axum::{routing::get, ApiRouter, IntoApiResponse}, openapi::{License, OpenApi, Tag}, transform::TransformOpenApi};
use axum::{response::{Html, IntoResponse}, Extension, Json};

pub(crate) fn docs_routes() -> ApiRouter {
    aide::gen::infer_responses(true);

    let router: ApiRouter = ApiRouter::new()
        .route("/scalar", get(serve_scalar))
        .route("/api_spec.json", get(serve_docs));

    aide::gen::infer_responses(false);

    router
}

async fn serve_docs(
    Extension(api): Extension<Arc<OpenApi>>
) -> impl IntoApiResponse {
    Json(api).into_response()
}

pub fn generate_api() -> OpenApi {
    aide::gen::on_error(|error| {
         log::error!("{error}")
    });

    aide::gen::extract_schemas(true);

    OpenApi::default()
}

pub fn configure_api(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Feed621 documentation")
        .description("REST API that looks like feed for e621.net")
        .version("1.0.0")
        .license(License {
            name: "hash".into(),
            identifier: Some("MIT".into()),
            url: Some("https://github.com/h4-h/feed621/blob/main/LICENSE".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "Test routes".into(),
            description: Some("There i'm trying to understand how axum and aide works.".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "User routes".into(),
            description: Some("Routes responsible to the operations with user.".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "Topic routes".into(),
            description: Some("Routes responsible to the operations with topics.".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "Subscription routes".into(),
            description: Some("Routes responsible to the operations with subscripbtions.".into()),
            ..Default::default()
        })
}

// BuT tHerE aRe alReAdy A ScAlArR in AiDe
// Yeah, pretty broken and outdated.
async fn serve_scalar() -> impl IntoApiResponse {
    Html(r#"
        <html>
          <head>
            <title>Feed621 documentation</title>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
          </head>
          <body>
            <script id="api-reference" data-url="/docs/api_spec.json"></script>
            <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference@1.24.33/dist/browser/standalone.min.js"></script>
          </body>
        </html>
    "#)
}
