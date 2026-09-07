use std::error::Error;

use axum::{Json, Router, response::IntoResponse, routing::post};
use axum_error_sets::{
    ApiResult, ResultStatusExt,
    codes::{Conflict, Forbidden, Internal},
};

fn main() {
    let _: Router<()> = Router::new().route("/", post(handler));
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
