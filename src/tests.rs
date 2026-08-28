use super::*;
use crate::code::{Conflict, InternalServerError, NotAcceptable, NotFound};
use axum::response::IntoResponse as _;
use axum_typed_routing::api_route;
use http::StatusCode;
use rootcause::Report;

struct MyApiError {
    report: Option<Report>,
    msg: Option<String>,
}

impl MyApiError {
    fn new(msg: impl Into<String>) -> Self {
        MyApiError {
            report: None,
            msg: Some(msg.into()),
        }
    }
}

impl<T: Into<Report>> From<T> for MyApiError {
    fn from(err: T) -> Self {
        MyApiError {
            report: Some(err.into()),
            msg: None,
        }
    }
}

impl ApiErrorValue for MyApiError {
    fn into_response_with(self, status: StatusCode) -> Response {
        let msg = self.msg.unwrap_or_else(|| "".to_string());
        (status, msg).into_response()
    }
}

impl AideApiErrorValue for MyApiError {
    type Inner = String;

    fn inferred_response_for(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut aide::openapi::Operation,
        status: StatusCode,
    ) -> aide::openapi::Response {
        aide::openapi::Response {
            description: format!("An error with status {}", status),
            ..Default::default()
        }
    }
}

type ApiResult<T, E> = Result<T, ApiError<MyApiError, E>>;

#[api_route(GET "/test")]
#[axum::debug_handler]
async fn test() -> ApiResult<(), (NotFound, InternalServerError)> {
    another_fn().into_not_found()?;
    another_fn().into_internal()?;
    // another_fn().status_conflict()?;

    subset().into_superset()?;
    // unhandled().into_superset()?;

    Err(NotFound(MyApiError::new("Not found")))?;
    // Err(Conflict(MyApiError::from_msg("Not found")))?;

    Err(ApiError::new_with::<NotFound>(MyApiError::new("Not found")))?;
    // Err(ApiError::new::<Conflict>(MyApiError::from_msg("Not found")))?;

    Ok(())
}

fn another_fn() -> Result<(), std::fmt::Error> {
    Ok(())
}

fn subset() -> ApiResult<(), (NotFound,)> {
    Ok(())
}

fn unhandled() -> ApiResult<(), (Conflict, NotAcceptable)> {
    Ok(())
}
