use std::{borrow::Cow, collections::HashMap};
use axum::{http::StatusCode, response::IntoResponse, Json};

/// Error wrapper that used between layers.
#[derive(Debug, thiserror::Error)]
pub(crate) enum AppError {
    /// `401 Unauthorized`.
    #[error("Authentication required.")]
    Unauthorized,
    /// `403 Forbidden`.
    #[error("User may not perform that action.")]
    Forbidden,
    /// `404 Not found`.
    #[error("You entered the V O I D 0w0")]
    NotFound,
    /// `422 Unprocessible entity`.
    ///
    /// `Self::UnprocessibleEntity.0` (error body) indicates what caused 422 response.
    #[error("Error in the request body.")]
    UnprocessibleEntity(HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>),
    /// `500 Internal server error`.
    ///
    /// Doesn't return actual error message to user for security reasons.
    #[error("Database error.")]
    Database(#[from] sqlx::Error),
    #[error("Internal server error ˙◠˙.")]
    /// `500 Internal server error`.
    ///
    /// Doesn't return actual error message to user for security reasons.
    /// Work like fallback.
    Internal(#[from] anyhow::Error),
}

impl AppError {
    pub fn unprocessible_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>> {
         Self::UnprocessibleEntity(errors.into_iter()
            .fold(HashMap::new(), |mut acc, (key, val)| {
                acc.entry(key.into()).or_default().push(val.into());
                acc
            })
        )
    }
    
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessibleEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Database(_) |
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::UnprocessibleEntity(errors) => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
                }
                
                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response();
            },
            Self::Database(ref e) => {
                log::error!("SQLx error: {:?}", e);
            },
            Self::Internal(ref e) => {
                log::error!("Generic error: {:?}", e);
            },
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}
