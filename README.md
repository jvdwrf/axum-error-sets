# axum-error-sets

[![Crates.io](https://img.shields.io/crates/v/axum-error-sets.svg)](https://crates.io/crates/axum-error-sets)
[![Documentation](https://docs.rs/axum-error-sets/badge.svg)](https://docs.rs/axum-error-sets)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)

Typed, composable HTTP error sets for [axum](https://github.com/tokio-rs/axum), with OpenAPI generation through [aide](https://github.com/tamasfe/aide).

Instead of one large error enum per application, each function lists the exact HTTP status codes it can return, as a tuple in its return type, such as `ApiResult<T, (Unauthorized, NotFound<String>)>`. The sets are built on [`type-sets`](https://docs.rs/type-sets/).

## Features

- **Exact error contracts:** each function declares which status codes it can return. Returning any other code is a compile error.
- **Every status code:** there is a wrapper type for every 4xx and 5xx code, with any `IntoResponse` type as the body: `NotFound`, `NotFound<String>`, `NotFound<Json<MyError>>`.
- **`?` just works:** status codes convert into any error set that contains them, and small sets grow into larger ones with `.into_superset()`.
- **Error wrapping helpers:** `.with_status::<BadRequest>()`, `.into_status::<..>()`, `.change_status::<..>()` and `.map_status(..)` on any `Result`.
- **OpenAPI support:** with the `aide` feature, every status code in a handler's set is documented in the generated OpenAPI spec.

## Installation

```toml
[dependencies]
axum-error-sets = "0.4"

# For OpenAPI generation with aide:
axum-error-sets = { version = "0.4", features = ["aide"] }
```

## Example

```rust
use axum::Json;
use axum_error_sets::{
    ApiResult, ApiResultExt as _, ResultStatusExt as _,
    codes::{Internal, NotFound, Unauthorized},
};

fn check_token(token: &str) -> Result<(), Unauthorized> {
    if token.is_empty() {
        return Err(Unauthorized(()));
    }
    Ok(())
}

fn find_user(id: u32) -> ApiResult<String, (NotFound<String>,)> {
    let name = lookup(id)
        .ok_or("no such user")
        .into_status::<NotFound, String>()?; // `&str` error -> `NotFound<String>`
    Ok(name)
}

async fn get_user(
    token: String,
    id: u32,
) -> ApiResult<Json<String>, (Unauthorized, NotFound<String>, Internal<String>)> {
    check_token(&token)?;                        // `Unauthorized` is in the set
    let name = find_user(id).into_superset()?;   // `(NotFound<String>,)` is a subset
    let name = normalize(name).with_status::<Internal>()?; // `String` error -> `Internal<String>`
    Ok(Json(name))
}
```

The [`examples`](./examples) directory has runnable examples, including handlers, aide integration, and use with axum-typed-routing.

## Pairs well with axum-typed-routing

[axum-typed-routing](https://github.com/jvdwrf/axum-typed-routing) lets you declare a route's path and parameters next to its handler, checked at compile time. Combined with its `api_route` macro, a handler's error set appears in the OpenAPI documentation with no extra annotations:

```rust
#[api_route(GET "/item/{id}")]
async fn get_item(id: u32) -> ApiResult<Json<Item>, (Unauthorized, NotFound<String>)> {
    // ...
}
```

See [`examples/5_typed_routing.rs`](./examples/5_typed_routing.rs).

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
