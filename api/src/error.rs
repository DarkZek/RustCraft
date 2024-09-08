use std::fmt::{Display, Formatter};
use axum::http::{StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct ErrorResponse {
    message: String
}

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        if self.0.downcast_ref::<ClientErr>().is_some() {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    message: self.0.to_string()
                }),
            ).into_response()
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: format!("Something went wrong: {}", self.0)
                }),
            ).into_response()
        }
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Error, Debug)]
pub struct ServerErr(pub String);

impl Display for ServerErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Error, Debug)]
pub struct ClientErr(pub String);

impl Display for ClientErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
