use anyhow::{Result, anyhow};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct HttpError(anyhow::Error);

impl HttpError {
    pub fn new(e: String) -> Self {
        HttpError(anyhow!(e))
    }
}

pub fn mkerr<T>(e: String) -> HttpResult<T> {
    Err(HttpError::new(e))
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for HttpError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub type HttpResult<T> = Result<T, HttpError>;
