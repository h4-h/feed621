use axum_typed_routing::{api_route, TypedApiRouter};
use crate::domain::ServerInfo;

mod openapi;
mod users;
mod topics;
mod subscriptions;

pub fn app_router() -> axum::Router {
    let mut api = aide::openapi::OpenApi::default();

    aide::axum::ApiRouter::new()
        .nest("/docs", openapi::router())
        .nest("/users", users::router())
        .nest("/topics", topics::router())
        .nest("/subscriptions", subscriptions::router())
        .typed_api_route(get_info)
        .finish_api_with(&mut api, openapi::configure_info)
        .layer(axum::Extension(std::sync::Arc::new(api)))
        .fallback(|| async { "You entered the V O I D 0w0" })
}

#[api_route(GET "/info" {
    description: "View information about server in format of `{pkg_version}-{backend_library}`.",
    tags: ["utils"],
    transform: |op| { op.response_with::<200, String, _>(|res| {
        res.description("Package version and backend library.").example("1.0.0-axum")
    }) }
})]
pub async fn get_info() -> String {
    crate::axum::AxumApi::get_server_info()
}
