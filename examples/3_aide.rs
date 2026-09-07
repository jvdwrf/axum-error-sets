use aide::{
    OperationOutput,
    axum::{ApiRouter, routing::post},
    openapi::OpenApi,
};
use axum::{Json, Router, response::IntoResponse};
use axum_error_sets::{
    ApiResult, ResultStatusExt,
    codes::{Conflict, Forbidden, Internal},
};
use std::error::Error;

fn main() {
    let mut api = OpenApi::default();

    let _: Router<()> = ApiRouter::new()
        .api_route("/", post(handler))
        .finish_api(&mut api);

    println!("{:#?}", api);
}

async fn handler() -> ApiResult<
    (),
    (
        Forbidden<Json<()>>,
        Conflict<Json<()>>,
        Internal<MyApiError>,
    ),
> {
    if some_condition() {
        return Err(Forbidden(Json(())).into());
    }

    // Simple wrapping with status code
    json_err().with_status::<Forbidden>()?;
    json_err().with_status::<Conflict>()?;

    // Wrapping + into conversion
    dyn_err().into_status::<Internal, _>()?;

    Ok(())
}

fn some_condition() -> bool {
    true
}

fn json_err() -> Result<(), Json<()>> {
    Ok(())
}

fn dyn_err() -> Result<(), Box<dyn Error + Send>> {
    Ok(())
}

struct MyApiError {
    err: Box<dyn Error + Send>,
}

impl IntoResponse for MyApiError {
    fn into_response(self) -> axum::response::Response {
        println!("[WARN] Internal server error: {}", self.err);
        "Internal server error".into_response()
    }
}

impl<T: Into<Box<dyn Error + Send>>> From<T> for MyApiError {
    fn from(err: T) -> Self {
        MyApiError { err: err.into() }
    }
}

impl OperationOutput for MyApiError {
    type Inner = &'static str;

    fn operation_response(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Option<aide::openapi::Response> {
        Some(aide::openapi::Response {
            description: "Internal server error".to_string(),
            ..Default::default()
        })
    }
}
