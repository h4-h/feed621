use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema)]
#[schema(title = "Error model")]
pub(crate) struct AppErrorResponse {
    #[schema(example = 404)]
    code: u16,
    #[schema(example = "Not found.")]
    body: String,
}

impl AppErrorResponse {
    pub fn not_found<T: std::fmt::Display>(cause: T) -> Self {
        Self {
            code: StatusCode::NOT_FOUND.as_u16(),
            body: format!("{cause} not found."),
        }
    }
}

impl std::fmt::Display for AppErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl axum::response::IntoResponse for AppErrorResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json::from(self).into_response()
    }
}
