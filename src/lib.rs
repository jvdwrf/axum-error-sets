use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use type_sets::{Contains, Superset};

/// A convenient alias for a `Result` type where the error is an [`ApiResponse`].
pub type ApiResult<T, S> = Result<T, ApiResponse<S>>;

/// A response that may be returned from an `axum` handler, with a flexible set of response types.
///
/// It implements
/// - [`IntoResponse`]
/// - Optionally [`aide::OperationOutput`] when the `aide` feature is enabled, and all responses implement `OperationOutput` as well.
/// - Optionally [`utoipa::ToResponse`] when the `utoipa` feature is enabled, and all responses implement `ToResponse` as well.
///
/// Simply specify the set of response types that this handler may return as a tuple in the `ApiResponse` type:
/// - `ApiResponse<(BadRequest<String>,)>`
/// - `ApiResponse<(BadRequest<String>, Internal<String>)>`
/// - `ApiResponse<(BadRequest<String>, Internal<String>, Forbidden<String>)>`
///
/// `ApiResponse<S>` implements `From<T>` for any `T` [contained](`type_sets::Contains`)
/// in the set `S`, allowing for easy error propagation.
/// See [`ResultStatusExt`] for convenient methods to help with error propagation.
///
/// Normally, this type is used as `Result<T, ApiResponse<S>>` (or the type-alias [`ApiResult`]). It can also be used itself as a direct return type from an `axum` handler.
///
/// # Examples
/// ```rust
/// # use axum_error_sets::{ApiResponse, codes::*};
/// // An very basic axum-handler. See the `examples` directory for more complete examples.
/// async fn handler() -> Result<(), ApiResponse<(NotFound<String>, BadRequest<String>)> { todo!() }
/// ```
pub struct ApiResponse<S> {
    response: Response,
    code: StatusCode,
    _marker: std::marker::PhantomData<fn() -> S>,
}

impl<S> ApiResponse<S> {
    /// Create a new `ApiResponse` from a status wrapper.
    pub fn new<T>(wrapper: T) -> Self
    where
        S: Contains<T>,
        T: StatusProvider<Inner: IntoResponse>,
    {
        Self::new_unchecked(wrapper.into_inner(), T::STATUS_CODE)
    }

    /// Create a new `ApiResponse` from a raw response and status code, without checking if it is valid
    pub fn new_unchecked(response: impl IntoResponse, code: StatusCode) -> Self {
        Self {
            response: response.into_response(),
            code,
            _marker: std::marker::PhantomData,
        }
    }

    /// Decompose the `ApiResponse` into its raw response and status code.
    pub fn into_parts(self) -> (Response, StatusCode) {
        (self.response, self.code)
    }

    /// Convert this `ApiResponse` into a new `ApiResponse` with a superset of the original error types.
    pub fn into_superset<U>(self) -> ApiResponse<U>
    where
        U: Superset<S>,
    {
        ApiResponse {
            response: self.response,
            code: self.code,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S> IntoResponse for ApiResponse<S> {
    fn into_response(self) -> Response {
        (self.code, self.response).into_response()
    }
}

impl<S> std::fmt::Debug for ApiResponse<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiResponse")
            .field("response", &self.response)
            .field("code", &self.code)
            .finish()
    }
}

macro_rules! utoipa_aide_impls {
    ($($n:tt => ($($E:ident),*)),+ $(,)?) => {
        $(
            #[allow(unused)]
            #[cfg(feature = "aide")]
            impl<$($E),*> aide::OperationOutput for ApiResponse<($($E,)*)>
            where
                $(
                    $E: StatusProvider<Inner: aide::OperationOutput>,
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

        $(
            #[allow(unused)]
            #[cfg(feature = "utoipa")]
            impl<$($E),*> utoipa::IntoResponses for ApiResponse<($($E,)*)>
            where
                $(
                    $E: StatusProvider<Inner: utoipa::IntoResponses>,
                )*
            {
                fn responses() -> std::collections::BTreeMap<
                    String,
                    utoipa::openapi::RefOr<utoipa::openapi::Response>,
                > {
                    let mut responses = std::collections::BTreeMap::new();

                    $(
                        let inner_responses =
                            <$E::Inner as utoipa::IntoResponses>::responses();

                        if let Some((_, response)) = inner_responses.into_iter().next() {
                            responses.insert(
                                $E::STATUS_CODE.as_u16().to_string(),
                                response,
                            );
                        }
                    )*

                    responses
                }
            }
        )+
    };
}

utoipa_aide_impls!(
    0 => (),
    1 => (E1),
    2 => (E1, E2),
    3 => (E1, E2, E3),
    4 => (E1, E2, E3, E4),
    5 => (E1, E2, E3, E4, E5),
    6 => (E1, E2, E3, E4, E5, E6),
    7 => (E1, E2, E3, E4, E5, E6, E7),
    8 => (E1, E2, E3, E4, E5, E6, E7, E8),
    9 => (E1, E2, E3, E4, E5, E6, E7, E8, E9),
    10 => (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10),
    11 => (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11),
    12 => (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12),
    13 => (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13),
    14 => (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14),
    15 => (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15),
    16 => (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16),
);

/// A [`StatusCode`] that wraps an inner value, and can thus be used for easy error
/// propagation while keeping track of the status-code in the type-system.
///
/// This trait is implemented for all codes in the [`codes`] module.
pub trait StatusProvider: From<Self::Inner> + Sized {
    /// The status code associated with type.
    const STATUS_CODE: StatusCode;

    /// The inner value type that is wrapped by this status wrapper.
    type Inner;

    type WithInner<T>: StatusProvider<Inner = T>;

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

    /// Apply a function to the inner value of this status wrapper,
    /// producing a new wrapper with the transformed inner value.
    fn map<T>(self, f: impl FnOnce(Self::Inner) -> T) -> Self::WithInner<T> {
        <Self::WithInner<T> as From<T>>::from(f(self.into_inner()))
    }
}

/// Extension trait for `Result` that provides methods to wrap errors with specific HTTP status codes.
pub trait ResultStatusExt<T, E>: Sized {
    /// Wraps the error `E` with the gives status `S`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use axum_error_sets::{ApiResultExt as _, codes::BadRequest};
    ///
    /// let result: Result<(), String> = Err("error".into());
    ///
    /// // Has type `Result<(), BadRequest<String>>``
    /// let _wrapped = result.with_status::<BadRequest>();
    /// ```
    fn with_status<S>(self) -> Result<T, S::WithInner<E>>
    where
        S: StatusProvider<Inner = ()>;

    /// Wraps the error `E` with the gives status `S`, while converting the inner error-type using [`Into::into`] to `E2`.
    ///
    /// Usually the second generic can be inferred by the compiler.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use axum_error_sets::{ApiResultExt as _, codes::BadRequest};
    ///
    /// let result: Result<(), &str> = Err("error");
    ///
    /// // Has type `Result<(), BadRequest<String>>`
    /// let _wrapped = result.into_status::<BadRequest, String>();
    /// ```
    fn into_status<S, E2>(self) -> Result<T, S::WithInner<E2>>
    where
        S: StatusProvider<Inner = ()>,
        E: Into<E2>;

    /// Changes the status of the error to the given [`StatusProvider`] `S`, preserving the inner error.
    fn change_status<S>(self) -> Result<T, S::WithInner<E::Inner>>
    where
        E: StatusProvider,
        S: StatusProvider;

    /// Maps the inner state of the given [`StatusProvider`] using the provided function `f`.
    fn map_status<F, O>(self, f: F) -> Result<T, E::WithInner<O>>
    where
        E: StatusProvider,
        F: FnOnce(E::Inner) -> O;

    /// Maps the inner error of the given [`StatusProvider`] using the [`Into::into`] conversion.
    fn map_status_into<O>(self) -> Result<T, E::WithInner<O>>
    where
        E: StatusProvider,
        E::Inner: Into<O>,
    {
        self.map_status(Into::into)
    }
}

impl<T, E> ResultStatusExt<T, E> for Result<T, E> {
    fn into_status<S, E2>(self) -> Result<T, S::WithInner<E2>>
    where
        S: StatusProvider<Inner = ()>,
        E: Into<E2>,
    {
        match self {
            Ok(val) => Ok(val),
            Err(e) => Err(S::WithInner::from(e.into())),
        }
    }

    fn with_status<S>(self) -> Result<T, S::WithInner<E>>
    where
        S: StatusProvider<Inner = ()>,
    {
        match self {
            Ok(val) => Ok(val),
            Err(e) => Err(S::WithInner::from(e)),
        }
    }

    fn map_status<F, O>(self, f: F) -> Result<T, E::WithInner<O>>
    where
        E: StatusProvider,
        F: FnOnce(E::Inner) -> O,
    {
        match self {
            Ok(val) => Ok(val),
            Err(e) => Err(e.map(f)),
        }
    }

    fn change_status<S>(self) -> Result<T, S::WithInner<E::Inner>>
    where
        E: StatusProvider,
        S: StatusProvider,
    {
        match self {
            Ok(val) => Ok(val),
            Err(e) => Err(S::WithInner::from(e.into_inner())),
        }
    }
}

/// Extension trait for `Result` types with `ApiResponse` errors, providing a method to convert the error type into a superset error type.
pub trait ApiResultExt<T, S> {
    /// Converts the error type of the `Result` into a superset error type.
    fn into_superset<U>(self) -> Result<T, ApiResponse<U>>
    where
        U: Superset<S>;
}

impl<T, S> ApiResultExt<T, S> for Result<T, ApiResponse<S>> {
    fn into_superset<U>(self) -> Result<T, ApiResponse<U>>
    where
        U: Superset<S>,
    {
        match self {
            Ok(val) => Ok(val),
            Err(e) => Err(e.into_superset()),
        }
    }
}

pub mod codes;
