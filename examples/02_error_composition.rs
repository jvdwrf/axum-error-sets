mod common;
use axum_error_sets::{
    ResultSetExt, StatusResultExt,
    code::{Conflict, InternalServerError, NotFound, Unauthorized},
};
use common::{AppResultSet, StringError};
use rootcause::report;

fn app_error(msg: &'static str) -> Result<String, StringError> {
    Err(StringError::new(msg))
}

fn generic_io_error() -> Result<(), std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::ConnectionReset,
        "database socket reset",
    ))
}

// -----------------------------------------------------------------------------
// Lower-Level Functions (Small Error Sets)
// -----------------------------------------------------------------------------

/// Database lookup function: only ever yields a 404 (NotFound).
fn fetch_user(id: &str) -> AppResultSet<String, (NotFound,)> {
    if id != "valid_id" {
        return app_error("user record not found")
            .into_not_found()
            .map_err(Into::into);
    }
    Ok(String::from("Alice"))
}

/// Authentication check: only ever yields a 401 (Unauthorized).
fn check_auth(token: &str) -> AppResultSet<(), (Unauthorized,)> {
    if token.is_empty() {
        app_error("missing auth token").into_unauthorized()?;
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// Mid/High-Level Service (Growing Error Sets)
// -----------------------------------------------------------------------------

/// Service layer combining lower-level functions.
///
/// `into_superset()` expands both `(Unauthorized,)` and `(NotFound,)`
/// into the larger `(Unauthorized, NotFound, Conflict, InternalServerError)` set.
fn update_user_profile(
    id: &str,
    token: &str,
    new_name: &str,
) -> AppResultSet<String, (Unauthorized, NotFound, Conflict, InternalServerError)> {
    // 1. Promote (Unauthorized,) to full set
    check_auth(token).into_superset()?;

    // 2. Promote (NotFound,) to full set
    let mut username = fetch_user(id).into_superset()?;

    // 3. Directly introduce 409 (Conflict) at this layer
    if new_name == "taken_username" {
        return Err(Conflict(report!("username already taken")).into());
    }

    // 4. Automatically wrap external errors into 500 (InternalServerError)
    generic_io_error().into_internal()?;

    username.push_str(" -> ");
    username.push_str(new_name);

    Ok(username)
}

fn main() {
    println!(
        "Failed Auth: {:?}",
        update_user_profile("valid_id", "", "NewName")
    );
    println!(
        "Failed Fetch: {:?}",
        update_user_profile("invalid_id", "token123", "NewName")
    );
    println!(
        "Conflict Error: {:?}",
        update_user_profile("valid_id", "token123", "taken_username")
    );
}
