use super::*;
use std::collections::BTreeMap;
use utoipa::openapi::RefOr;

macro_rules! impl_into_responses {
    ($($error:ident),+ $(,)?) => {
        impl<R, $($error),+> utoipa::IntoResponses for ErrorSet<R, ($($error,)+)>
        where
            R: UtoipaErrorSetValue,
            $($error: StatusWrapper),+
        {
            fn responses() -> BTreeMap<String, RefOr<utoipa::openapi::Response>> {
                let mut map = BTreeMap::new();
                $(
                    map.insert(
                        $error::STATUS_CODE.as_u16().to_string(),
                        RefOr::T(R::response_for($error::STATUS_CODE)),
                    );
                )+
                map
            }
        }
    };
}

impl_into_responses!(E1);
impl_into_responses!(E1, E2);
impl_into_responses!(E1, E2, E3);
impl_into_responses!(E1, E2, E3, E4);
impl_into_responses!(E1, E2, E3, E4, E5);
impl_into_responses!(E1, E2, E3, E4, E5, E6);
impl_into_responses!(E1, E2, E3, E4, E5, E6, E7);
impl_into_responses!(E1, E2, E3, E4, E5, E6, E7, E8);
impl_into_responses!(E1, E2, E3, E4, E5, E6, E7, E8, E9);
impl_into_responses!(E1, E2, E3, E4, E5, E6, E7, E8, E9, E10);
impl_into_responses!(E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11);
impl_into_responses!(E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12);

impl<R> utoipa::IntoResponses for ErrorSet<R, ()>
where
    R: UtoipaErrorSetValue,
{
    fn responses() -> BTreeMap<String, RefOr<utoipa::openapi::Response>> {
        BTreeMap::new()
    }
}
