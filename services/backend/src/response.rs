use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub(crate) struct AppResponse<B> where B: Serialize {
    code: u16,
    body: B,
}

impl<B: Serialize> AppResponse<B> {
    pub fn success(body: B) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            body,
        }
    }

    pub fn created(body: B) -> Self {
        Self {
            code: StatusCode::CREATED.as_u16(),
            body,
        }
    }
}

impl AppResponse<String> {
    pub fn not_found<T: std::fmt::Display>(cause: T) -> Self {
        Self {
            code: StatusCode::NOT_FOUND.as_u16(),
            body: format!("{cause} not found."),
        }
    }
}

impl<T: Serialize + std::fmt::Debug> std::fmt::Display for AppResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: Serialize> axum::response::IntoResponse for AppResponse<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json::from(self).into_response()
    }
}
