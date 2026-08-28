mod common;
use aide::{
    axum::{
        ApiRouter,
        routing::{get_with, post_with},
    },
    openapi::OpenApi,
};
use axum::{Json, extract::Path};
use axum_error_sets::{
    ResultSetExt,
    code::{BadRequest, Conflict, InternalServerError, NotFound},
};
use common::{AppResultSet, StringError};
use rootcause::report;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// DTOs
// -----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, JsonSchema)]
struct User {
    id: String,
    name: String,
}

#[derive(Deserialize, JsonSchema)]
struct CreateUserPayload {
    name: String,
}

// -----------------------------------------------------------------------------
// Core Business Logic (Returns narrow error sets)
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
// Axum Handlers with Aide Support
// -----------------------------------------------------------------------------

/// GET /users/{id}
///
/// OpenAPI documents both 404 (NotFound) and 500 (InternalServerError).
async fn get_user_handler(
    Path(id): Path<String>,
) -> AppResultSet<Json<User>, (NotFound, InternalServerError)> {
    let user = find_user_by_id(&id).into_superset()?;
    Ok(Json(user))
}

/// POST /users
///
/// Combines payload validation (400), registration conflicts (409), and database failures (500).
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
// Router Setup & Main
// -----------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    let mut api = OpenApi::default();

    // ApiRouter automatically extracts openapi metadata from declared handler error sets via AideErrorSetValue
    let app = ApiRouter::<()>::new()
        .api_route(
            "/users/{id}",
            get_with(get_user_handler, |op| {
                op.description("Fetch a user by their unique identifier")
            }),
        )
        .api_route(
            "/users",
            post_with(create_user_handler, |op| {
                op.description("Register a new user")
            }),
        )
        .finish_api(&mut api);

    println!("OpenAPI Schema generated successfully!");
    println!(
        "Routes documented: {}",
        serde_json::to_string_pretty(&api).unwrap()
    );

    // Run axum server with `app` here
    let _ = app;
}
