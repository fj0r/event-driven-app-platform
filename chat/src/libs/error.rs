use anyhow::Result;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct HttpError(anyhow::Error);

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

pub type HttpResult<T> = Result<T, HttpError>;
