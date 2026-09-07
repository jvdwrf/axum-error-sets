use axum::Json;
use axum_error_sets::{
    ApiResult, ApiResultExt, ResultStatusExt as _,
    codes::{BadRequest, Forbidden, Internal},
};

fn main() -> ApiResult<
    (),
    (
        BadRequest<String>,
        Internal<Json<String>>,
        Forbidden<String>,
    ),
> {
    perform_action().with_status::<BadRequest>()?;
    perform_action2().into_status::<BadRequest, _>()?;

    perform_action()
        .with_status::<Internal>()
        .map_status(Json)?;
    perform_action2()
        .with_status::<Internal>()
        .map_status(|s| Json(s.to_string()))?;

    perform_action3()?;

    perform_action4().change_status::<Internal>()?;

    perform_actions().into_superset()?;

    Ok(())
}

fn perform_action() -> Result<(), String> {
    Ok(())
}

fn perform_action2() -> Result<(), &'static str> {
    Ok(())
}

fn perform_action3() -> Result<(), Forbidden<String>> {
    Ok(())
}

fn perform_action4() -> Result<(), Forbidden<Json<String>>> {
    Ok(())
}

fn perform_actions() -> ApiResult<(), (BadRequest<String>, Internal<Json<String>>)> {
    Ok(())
}
