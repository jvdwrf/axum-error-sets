# axum-error-sets

[![Crates.io](https://img.shields.io/crates/v/axum-error-sets.svg)](https://crates.io/crates/axum-error-sets)
[![Documentation](https://docs.rs/axum-error-sets/badge.svg)](https://docs.rs/axum-error-sets)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)

Typed, composable HTTP error sets for [Axum](https://github.com/tokio-rs/axum), with OpenAPI generation through [Aide](https://github.com/tamasfe/aide) or [Utoipa](https://github.com/juhaku/utoipa)

Instead of monolithic error enums or loosely-typed responses, functions declare the exact set of HTTP status codes they can return using type-level tuple sets (e.g., `(NotFound, Unauthorized)`), powered by [`type-sets`](https://docs.rs/type-sets/).

---

## Features

- **No Monolithic Error Enums:** Avoid constructing domain-wide error enums or per-function error types.
- **Exact Error Contracts:** Functions declare precisely which HTTP status codes they can produce.
- **Subset-to-Superset Promotion:** Error sets grow deterministically as they move up application layers via `.into_superset()`.
- **Custom Response Formatting:** Implement `IntoResponseWith` to control how error values convert into Axum responses.
- **Compile-Time Guarantees:** Returning undeclared status codes produces a compiler error.
- **OpenAPI / Aide Support:** Implement `AideResponseFor` to automatically document status codes in OpenAPI specifications.

---

## Quick Example

For complete, runnable code, see the [`examples/`](./examples) directory.

```rust
use axum_error_sets::{
    ApiResultExt, StatusResultExt,
    code::{Conflict, InternalServerError, NotFound, Unauthorized},
};
use common::{AppResultSet, StringError};

fn fetch_user(id: &str) -> AppResultSet<String, (NotFound,)> {
    if id != "valid_id" {
        return Err(StringError::new("user record not found")).into_not_found();
    }
    Ok(String::from("Alice"))
}

fn check_auth(token: &str) -> AppResultSet<(), (Unauthorized,)> {
    if token.is_empty() {
        return Err(StringError::new("missing auth token")).into_unauthorized();
    }
    Ok(())
}

// Low-level error sets expand into a larger contract via `.into_superset()`
fn update_user_profile(
    id: &str,
    token: &str,
    new_name: &str,
) -> AppResultSet<String, (Unauthorized, Conflict, InternalServerError, NotFound)> {
    check_auth(token).into_superset()?;
    let mut username = fetch_user(id).into_superset()?;

    if new_name == "taken_username" {
        return Err(StringError::new("username taken")).into_conflict()?;
    }

    username.push_str(" -> ");
    username.push_str(new_name);
    Ok(username)
}