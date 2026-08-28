use axum::response::{IntoResponse, Response};
use axum_error_sets::{AideResponseFor, ErrorSet, IntoResponseWith, UtoipaResponseFor};
use http::StatusCode;
use rootcause::Report;

#[derive(Debug)]
pub struct AppError {
    pub message: Report,
}

impl AppError {
    pub fn new(msg: impl Into<Report>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl<T: Into<Report>> From<T> for AppError {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl IntoResponseWith for AppError {
    fn into_response_with(self, status: StatusCode) -> Response {
        (status, format!("{}", self.message)).into_response()
    }
}

impl AideResponseFor for AppError {
    type Inner = String;

    fn inferred_response_for(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut aide::openapi::Operation,
        status: StatusCode,
    ) -> aide::openapi::Response {
        aide::openapi::Response {
            description: format!("Request failed with status {status}"),
            ..Default::default()
        }
    }
}

impl UtoipaResponseFor for AppError {
    fn response_for(status: StatusCode) -> utoipa::openapi::Response {
        utoipa::openapi::Response::new(format!("Request failed with status {status}"))
    }
}

pub type AppResultSet<T, E> = Result<T, ErrorSet<AppError, E>>;

pub struct StringError {
    pub message: String,
}

impl StringError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl std::error::Error for StringError {}

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Debug for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StringError({})", self.message)
    }
}
