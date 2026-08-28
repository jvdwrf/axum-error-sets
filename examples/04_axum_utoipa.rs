mod common;

use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
};
use axum_error_sets::{
    ErrorSet, ResultSetExt,
    code::{BadRequest, Conflict, InternalServerError, NotFound},
};
use common::{AppResultSet, StringError};
use rootcause::report;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::common::AppError;

// -----------------------------------------------------------------------------
// DTOs (using utoipa::ToSchema instead of schemars::JsonSchema)
// -----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, ToSchema, OpenApi)]
struct User {
    id: String,
    name: String,
}

#[derive(Deserialize, ToSchema, OpenApi)]
struct CreateUserPayload {
    name: String,
}

// -----------------------------------------------------------------------------
// Core Business Logic
// -----------------------------------------------------------------------------

fn find_user_by_id(id: &str) -> AppResultSet<User, (NotFound,)> {
    if id != "42" {
        return Err(NotFound(StringError::new("user ID not found")).into());
    }
    Ok(User {
        id: id.to_string(),
        name: String::from("Alice"),
    })
}

fn create_user_in_db(name: &str) -> AppResultSet<User, (Conflict, InternalServerError)> {
    if name == "admin" {
        return Err(Conflict(report!("username 'admin' is reserved")).into());
    }
    Ok(User {
        id: String::from("43"),
        name: name.to_string(),
    })
}

// -----------------------------------------------------------------------------
// Axum Handlers with Utoipa Support
// -----------------------------------------------------------------------------

/// GET /users/{id}
///
/// utoipa automatically expands `AppResultSet<Json<User>, ...>` via the `IntoResponses` trait.
#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = User),
        ErrorSet<AppError, (NotFound, InternalServerError)>
    )
)]
async fn get_user_handler(
    Path(id): Path<String>,
) -> AppResultSet<Json<User>, (NotFound, InternalServerError)> {
    let user = find_user_by_id(&id).into_superset()?;
    Ok(Json(user))
}

/// POST /users
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserPayload,
    responses(
        (status = 200, description = "User created successfully", body = User),

        ErrorSet<AppError, (NotFound, InternalServerError)>
    )
)]
async fn create_user_handler(
    Json(payload): Json<CreateUserPayload>,
) -> AppResultSet<Json<User>, (BadRequest, Conflict, InternalServerError)> {
    if payload.name.trim().is_empty() {
        return Err(BadRequest(StringError::new("name cannot be empty")).into());
    }

    let new_user = create_user_in_db(&payload.name).into_superset()?;
    Ok(Json(new_user))
}

// -----------------------------------------------------------------------------
// OpenAPI Documentation Container
// -----------------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(
    paths(get_user_handler, create_user_handler,),
    components(schemas(User, CreateUserPayload))
)]
struct ApiDoc;

// -----------------------------------------------------------------------------
// Router Setup & Main
// -----------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    let api = ApiDoc::openapi();

    let app = Router::<()>::new()
        .route("/users/{id}", get(get_user_handler))
        .route("/users", post(create_user_handler));

    println!("OpenAPI Schema generated successfully!");
    println!(
        "Routes documented:\n{}",
        serde_json::to_string_pretty(&api).unwrap()
    );

    let _ = app;
}
