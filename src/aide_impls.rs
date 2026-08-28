use super::*;

macro_rules! impl_operation_output {
    ($($error:ident),+ $(,)?) => {
        impl<R, $($error),+> aide::OperationOutput for ErrorSet<R, ($($error,)+)>
        where
            R: AideResponseFor,
            $($error: StatusWrapper),+
        {
            type Inner = R::Inner;

            fn operation_response(
                _ctx: &mut aide::generate::GenContext,
                _operation: &mut aide::openapi::Operation,
            ) -> Option<aide::openapi::Response> {
                None
            }

            fn inferred_responses(
                ctx: &mut aide::generate::GenContext,
                operation: &mut aide::openapi::Operation,
            ) -> Vec<(Option<u16>, aide::openapi::Response)> {
                vec![
                    $(
                        (
                            Some($error::STATUS_CODE.as_u16()),
                            R::inferred_response_for(
                                ctx,
                                operation,
                                $error::STATUS_CODE,
                            ),
                        ),
                    )+
                ]
            }
        }
    };
}

impl_operation_output!(E1);
impl_operation_output!(E1, E2);
impl_operation_output!(E1, E2, E3);
impl_operation_output!(E1, E2, E3, E4);
impl_operation_output!(E1, E2, E3, E4, E5);
impl_operation_output!(E1, E2, E3, E4, E5, E6);
impl_operation_output!(E1, E2, E3, E4, E5, E6, E7);
impl_operation_output!(E1, E2, E3, E4, E5, E6, E7, E8);
impl_operation_output!(E1, E2, E3, E4, E5, E6, E7, E8, E9);
impl_operation_output!(E1, E2, E3, E4, E5, E6, E7, E8, E9, E10);
impl_operation_output!(E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11);
impl_operation_output!(E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12);

impl<R> aide::OperationOutput for ErrorSet<R, ()>
where
    R: AideResponseFor,
{
    type Inner = R::Inner;

    fn operation_response(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Option<aide::openapi::Response> {
        None
    }

    fn inferred_responses(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        vec![]
    }
}
