mod common;

use axum_error_sets::{
    StatusResultExt,
    code::{InternalServerError, NotFound},
};
use common::{AppError, CommonApiResult as AppApiResult};

fn database_query() -> Result<String, AppError> {
    Err(AppError::new("record missing"))
}

// Maps standard Result<T, CommonError> to a 404 ApiError
fn get_user() -> AppApiResult<String, (NotFound,)> {
    let user = database_query().into_not_found()?;
    Ok(user)
}

// Maps multiple status codes within the same function
fn process_user() -> AppApiResult<String, (NotFound, InternalServerError)> {
    let user = database_query().into_not_found()?;

    if user.is_empty() {
        return Err(AppError::new("payload corrupt")).into_internal()?;
    }

    Ok(user)
}

fn main() {
    println!("get_user: {:?}", get_user());
    println!("process_user: {:?}", process_user());
}
