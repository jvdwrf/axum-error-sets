use axum::{
    Json,
    response::{IntoResponse as _, Response},
};
use axum_error_sets::{AideErrorSetValue, ErrorSet, ErrorSetValue, StatusResultExt as _};
use axum_error_sets::{
    ResultSetExt as _,
    code::{Conflict, InternalServerError, NotFound, Unauthorized},
};
use axum_typed_routing::api_route;
use http::StatusCode;
use rootcause::Report;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use type_sets::SupersetOf;

#[derive(Debug)]
struct AppError {
    report: Option<Report>,
    message: Option<String>,
}

impl AppError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            report: None,
            message: Some(message.into()),
        }
    }
}

impl<T: Into<Report>> From<T> for AppError {
    fn from(error: T) -> Self {
        Self {
            report: Some(error.into()),
            message: None,
        }
    }
}

impl ErrorSetValue for AppError {
    fn into_response_with(self, status: StatusCode) -> Response {
        let message = self.message.unwrap_or_default();
        (status, message).into_response()
    }
}

impl AideErrorSetValue for AppError {
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

type ApiResult<T, E> = Result<T, ErrorSet<AppError, E>>;

// =============================================================================
// The important part: error sets grow as they move up the application.
// =============================================================================

/// The repository only knows about `NotFound`.
///
/// This is the smallest error set in the example.
fn find_user() -> ApiResult<User, (NotFound,)> {
    Err(ErrorSet::new_with::<NotFound>(AppError::new(
        "user not found",
    )))
}

fn is_superset<T, R>()
where
    T: SupersetOf<R>,
{
}

fn is_subset<T, R>()
where
    T: type_sets::SubsetOf<R>,
{
}

/// The service adds `Conflict` to the repository's error set.
///
/// `(NotFound,)` can be expanded into `(NotFound, Conflict)`.
fn update_user() -> ApiResult<User, (NotFound, Conflict)> {
    find_user().into_superset()?;

    check_conflict().into_conflict()?;

    Ok(User)
}

/// The handler adds `Unauthorized` and `InternalServerError`.
///
/// The final API contract contains every error that can be produced
/// anywhere below it.
#[api_route(PUT "/users/{id}")]
#[axum::debug_handler]
async fn update(
    id: String,
) -> ApiResult<Json<User>, (Unauthorized, NotFound, Conflict, InternalServerError)> {
    authenticate().into_unauthorized()?;

    update_user().into_superset()?;

    persist().into_internal()?;

    Ok(Json(User))
}

// =============================================================================
// ResponseExt / StatusResultExt
// =============================================================================

/// A plain `Result` has no HTTP semantics.
///
/// `into_not_found()` converts its error into a typed `404`.
fn repository_example() -> ApiResult<User, (NotFound,)> {
    database_lookup().into_not_found().map_err(Into::into)
}

/// Once an error has been assigned a status, that status is retained.
///
/// `into_superset()` can add possible statuses, but it cannot remove them.
fn service_example() -> ApiResult<User, (NotFound, Conflict)> {
    repository_example().into_superset()
}

// =============================================================================
// Subsets can be expanded
// =============================================================================

/// A `(NotFound,)` result is a valid subset of `(NotFound, Conflict)`.
///
/// This allows lower-level functions to remain precise while callers
/// expose a larger API error contract.
fn subset_to_superset() -> ApiResult<User, (NotFound, Conflict)> {
    let user: ApiResult<User, (NotFound,)> = find_user();

    user.into_superset()
}

// =============================================================================
// But error sets can never be shrunk
// =============================================================================

/// This would NOT compile:
///
/// ```compile_fail
/// fn shrink() -> Result<User, (NotFound,)> {
///     let result: Result<User, (NotFound, Conflict)> = update_user();
///
///     // `Conflict` cannot disappear from the error set.
///     result.into_subset()
/// }
/// ```
///
/// There is deliberately no `into_subset()` operation.
///
/// Once a result can contain `Conflict`, the type system guarantees
/// that callers cannot pretend it can only contain `NotFound`.

// =============================================================================
// Every status must belong to the declared set
// =============================================================================

/// `into_conflict()` is valid because `Conflict` is part of the
/// function's declared error set.
fn valid_status() -> ApiResult<(), (NotFound, Conflict)> {
    check_conflict().into_conflict()?;
    Ok(())
}

/// This would NOT compile:
///
/// ```compile_fail
/// fn invalid_status() -> Result<(), (NotFound, Conflict)> {
///     database_lookup()
///         .into_forbidden()?;
///
///     Ok(())
/// }
/// ```
///
/// `Forbidden` isn't part of `(NotFound, Conflict)`, so the `?`
/// cannot convert the error into the function's declared error type.

// =============================================================================
// A status can only be introduced once
// =============================================================================

/// These are equivalent from the type system's perspective:
fn explicit_status() -> ApiResult<(), (NotFound,)> {
    database_lookup().into_not_found()?;
    Ok(())
}

fn propagated_status() -> ApiResult<(), (NotFound,)> {
    find_user()?;

    Ok(())
}

// =============================================================================
// Lower-level code doesn't need to know about the final API
// =============================================================================

fn database_lookup() -> std::result::Result<User, AppError> {
    Err(AppError::new("user not found"))
}

fn check_conflict() -> std::result::Result<(), AppError> {
    Ok(())
}

fn authenticate() -> std::result::Result<(), AppError> {
    Ok(())
}

fn persist() -> std::result::Result<(), AppError> {
    Ok(())
}

#[derive(JsonSchema, Serialize, Deserialize)]
struct User;

fn main() {}
