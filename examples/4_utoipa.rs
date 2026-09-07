// use axum::{Json, Router, response::IntoResponse, routing::post};
// use axum_error_sets::{
//     ApiResponse, ApiResult, ResultStatusExt,
//     codes::{Conflict, Forbidden, Internal},
// };
// use std::error::Error;
// use utoipa::{IntoResponses, PartialSchema, ToResponse, ToSchema, schema};
// use utoipa_axum::{router::OpenApiRouter, routes};

// fn main() {
//     let app = OpenApiRouter::<()>::new().routes(routes!(handler));

//     let api = app.into_openapi();
//     println!("{}", api.to_pretty_json().unwrap());
// }

// #[utoipa::path(
//     post,
//     path = "/",
//     responses(
//         // (status = 200, response = Json<()>),
//         ApiResponse<(
//             Forbidden<Json<()>>,
//             Conflict<Json<()>>,
//             Internal<MyApiError>,
//         )>,
//     )
// )]
// async fn handler() -> ApiResult<
//     (),
//     (
//         Forbidden<Json<()>>,
//         Conflict<Json<()>>,
//         Internal<MyApiError>,
//     ),
// > {
//     if some_condition() {
//         return Err(Forbidden(Json(())).into());
//     }

//     // Simple wrapping with status code
//     json_err().with_status::<Forbidden>()?;
//     json_err().with_status::<Conflict>()?;

//     // Wrapping + conversion into the API error type
//     dyn_err().into_status::<Internal, _>()?;

//     Ok(())
// }

// impl ToSchema for () {
//     fn name() -> std::borrow::Cow<'static, str> {
//         let full_type_name = std::any::type_name::<Self>();
//         let type_name_without_generic = full_type_name
//             .split_once("<")
//             .map(|(s1, _)| s1)
//             .unwrap_or(full_type_name);
//         let type_name = type_name_without_generic
//             .rsplit_once("::")
//             .map(|(_, tn)| tn)
//             .unwrap_or(type_name_without_generic);
//         std::borrow::Cow::Borrowed(type_name)
//     }

//     fn schemas(
//         schemas: &mut Vec<(
//             String,
//             utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
//         )>,
//     ) {
//         // nothing by default
//     }
// }

// fn some_condition() -> bool {
//     true
// }

// fn json_err() -> Result<(), Json<()>> {
//     Ok(())
// }

// fn dyn_err() -> Result<(), Box<dyn Error + Send>> {
//     Ok(())
// }

// struct MyApiError {
//     err: Box<dyn Error + Send>,
// }

// impl IntoResponse for MyApiError {
//     fn into_response(self) -> axum::response::Response {
//         println!("[WARN] Internal server error: {}", self.err);

//         "Internal server error".into_response()
//     }
// }

// impl<T: Into<Box<dyn Error + Send>>> From<T> for MyApiError {
//     fn from(err: T) -> Self {
//         Self { err: err.into() }
//     }
// }

// impl ToSchema for MyApiError {
//     fn name() -> std::borrow::Cow<'static, str> {
//         "MyApiError".into()
//     }

//     fn schemas(
//         schemas: &mut Vec<(
//             String,
//             utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
//         )>,
//     ) {
//         schemas.push((
//             "MyApiError".to_string(),
//             utoipa::openapi::RefOr::T(schema!(String).into()),
//         ));
//     }
// }

// fn into_responses<T: IntoResponses>() {}
// fn to_schema<T: ToSchema>() {}
// fn partial_schema<T: PartialSchema>() {}
// fn to_response<'a, T: ToResponse<'a>>() {}

// fn test() {
//     into_responses::<Json<()>>();
//     to_schema::<Json<()>>();
//     partial_schema::<Json<()>>();
//     to_response::<Json<()>>();
// }

fn main() {}
