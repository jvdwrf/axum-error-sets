use axum::response::{IntoResponse, Response};
use axum_error_sets::{AideApiErrorValue, ApiError, ApiErrorValue};
use http::StatusCode;

#[derive(Debug, Clone)]
pub struct AppError {
    pub message: String,
}

impl AppError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl ApiErrorValue for AppError {
    fn into_response_with(self, status: StatusCode) -> Response {
        (status, self.message).into_response()
    }
}

impl AideApiErrorValue for AppError {
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

/// Convenience alias for API results using CommonError
pub type CommonApiResult<T, E> = Result<T, ApiError<AppError, E>>;
