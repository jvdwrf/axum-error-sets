//! Using `axum-error-sets` together with `axum-typed-routing`.
use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::{Json, Router};
use axum_error_sets::{
    ApiResult, ResultStatusExt,
    codes::{NotFound, Unauthorized},
};
use axum_typed_routing::{TypedApiRouter, api_route};

/// Get an item
#[api_route(GET "/item/{id}?token")]
async fn get_item(
    id: u32,
    token: Option<String>,
) -> ApiResult<Json<String>, (Unauthorized, NotFound<String>)> {
    if token.is_none() {
        return Err(Unauthorized(()).into());
    }

    let item = find_item(id).with_status::<NotFound>()?;
    Ok(Json(item))
}

fn find_item(id: u32) -> Result<String, String> {
    Err(format!("item {id} does not exist"))
}

fn main() {
    let mut api = OpenApi::default();

    let _: Router<()> = ApiRouter::new()
        .typed_api_route(get_item)
        .finish_api(&mut api);

    // Documents 200, 401 and 404 for `GET /item/{id}`.
    println!("{}", serde_json::to_string_pretty(&api).unwrap());
}
