use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use type_sets::Contains;

pub type ApiResult<T, S> = Result<T, ApiResponse<S>>;

pub trait WrapsResponse: Sized {
    /// The status code associated with type.
    const STATUS_CODE: StatusCode;

    /// The inner value type that is wrapped by this status wrapper.
    type Inner;

    /// The pure type of this status wrapper, without any inner value.
    type Pure: WrapsResponse<Inner = ()>;

    /// Convert this status wrapper into its inner value.
    fn into_inner(self) -> Self::Inner;

    /// Convert this status wrapper into an [`ApiErrorSet`] with the
    /// given inner value type. (`into` can be used as well)
    fn into_set<E>(self) -> ApiResponse<E>
    where
        Self::Inner: IntoResponse,
        E: Contains<Self>,
    {
        ApiResponse::new(self)
    }
}

pub struct ApiResponse<S> {
    response: Response,
    code: StatusCode,
    _marker: std::marker::PhantomData<fn() -> S>,
}

impl<S> ApiResponse<S> {
    pub fn new<T>(wrapper: T) -> Self
    where
        S: Contains<T>,
        T: WrapsResponse<Inner: IntoResponse>,
    {
        Self {
            response: wrapper.into_inner().into_response(),
            code: T::STATUS_CODE,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S> IntoResponse for ApiResponse<S> {
    fn into_response(self) -> Response {
        (self.code, self.response).into_response()
    }
}

macro_rules! impl_operation_output {
    ($($n:tt => ($($E:ident),*)),+ $(,)?) => {
        $(
            #[cfg(feature = "aide")]
            impl<$($E),*> aide::OperationOutput for ApiResponse<($($E,)*)>
            where
                $(
                    $E: WrapsResponse<Inner: aide::OperationOutput>,
                )*
            {
                type Inner = (StatusCode, Response);

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
                        $((
                            Some($E::STATUS_CODE.as_u16()),
                            <$E::Inner as aide::OperationOutput>
                                ::operation_response(ctx, operation)
                                .unwrap_or_default(),
                        )),*
                    ]
                }
            }
        )+
    };
}

impl_operation_output!(
    0 => (),
    1 => (E1),
    2 => (E1, E2),
    3 => (E1, E2, E3),
    4 => (E1, E2, E3, E4),
    5 => (E1, E2, E3, E4, E5),
    6 => (E1, E2, E3, E4, E5, E6),
    7 => (E1, E2, E3, E4, E5, E6, E7),
    8 => (E1, E2, E3, E4, E5, E6, E7, E8),
);

pub mod codes;
