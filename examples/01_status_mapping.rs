mod common;

use axum_error_sets::{
    StatusResultExt,
    code::{InternalServerError, NotFound},
};
use common::{AppResultSet, StringError};

fn app_error() -> Result<String, StringError> {
    Err(StringError::new("record missing"))
}

fn generic_error() -> Result<(), std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "generic error",
    ))
}

// Maps standard Result<T, CommonError> to a 404 ApiError
fn get_user() -> AppResultSet<String, (NotFound,)> {
    let user = app_error().into_not_found()?;
    Ok(user)
}

// Maps multiple status codes within the same function
fn process_user() -> AppResultSet<String, (NotFound, InternalServerError)> {
    let user = app_error().into_not_found()?;

    if user.is_empty() {
        return Err(StringError::new("payload corrupt")).into_internal()?;
    }

    // Since AppError implements From<T> where T: Error, the ? operator automatically
    // converts the std::io::Error into an AppError.
    generic_error().into_internal()?;

    Ok(user)
}

fn main() {
    println!("get_user: {:?}", get_user());
    println!("process_user: {:?}", process_user());
}
