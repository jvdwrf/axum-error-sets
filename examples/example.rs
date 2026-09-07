use axum_error_sets::{
    ApiResult, ApiResultExt, ResultStatusExt as _,
    codes::{BadRequest, Forbidden, Internal},
};

fn main() {}

fn test() -> ApiResult<(), (BadRequest<String>, Internal<String>, Forbidden<String>)> {
    perform_action().with_status::<BadRequest>()?;
    perform_action2().into_status::<BadRequest, _>()?;

    perform_action().with_status::<Internal>()?;
    perform_action2().into_status::<Internal, _>()?;

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
    Err(Forbidden("error".into()))
}

fn perform_action4() -> Result<(), Forbidden<String>> {
    Err(Forbidden("error".into()))
}

fn perform_actions() -> ApiResult<(), (BadRequest<String>, Internal<String>)> {
    Ok(())
}
