//! Typed, composable HTTP error sets for [axum], with OpenAPI generation through
//! [aide](https://docs.rs/aide).
//!
//! Instead of one large error enum per application, each function lists the exact HTTP status
//! codes it can return, as a tuple in its return type:
//!
//! ```rust
//! # use axum_error_sets::{ApiResult, codes::{NotFound, Unauthorized}};
//! async fn get_user() -> ApiResult<String, (Unauthorized, NotFound<String>)> {
//!     # todo!()
//!     // ...
//! }
//! ```
//!
//! Returning a status code that isn't in the set is a compile error, and with the `aide`
//! feature every status code in the set appears in the generated OpenAPI documentation.
//!
//! # Building blocks
//! - **Status codes.** Every 4xx and 5xx status code has a wrapper type in [`codes`], such as
//!   [`NotFound<T>`](codes::NotFound). The wrapped value `T` is the response body and must
//!   implement [`IntoResponse`]. It defaults to `()`, which means an empty body.
//! - **Error sets.** [`ApiResponse<S>`] is an error response whose status code is one of the
//!   codes in the tuple `S`. [`ApiResult<T, S>`] is short for `Result<T, ApiResponse<S>>`.
//! - **`?` conversion.** Any status code `C` converts into `ApiResponse<S>` when `S` contains
//!   `C`, so `?` works directly. The order of the codes in the tuple doesn't matter.
//! - **Wrapping errors.** [`ResultStatusExt`] adds methods to every `Result` for giving its
//!   error a status code (`with_status`, `into_status`), changing it (`change_status`), or
//!   changing the body (`map_status`, `map_status_into`).
//! - **Growing sets.** [`ApiResultExt::into_superset`] turns a result with a small error set
//!   into one with a larger set, so functions with narrow sets can be called from functions
//!   with wider ones.
//!
//! # Example
//! ```rust
//! use axum::Json;
//! use axum_error_sets::{
//!     ApiResult, ApiResultExt as _, ResultStatusExt as _,
//!     codes::{Internal, NotFound, Unauthorized},
//! };
//!
//! fn check_token(token: &str) -> Result<(), Unauthorized> {
//!     if token.is_empty() {
//!         return Err(Unauthorized(()));
//!     }
//!     Ok(())
//! }
//!
//! fn find_user(id: u32) -> ApiResult<String, (NotFound<String>,)> {
//!     let name = lookup(id)
//!         .ok_or("no such user")
//!         // `&str` error -> `NotFound<String>`
//!         .into_status::<NotFound, String>()?;
//!     Ok(name)
//! }
//!
//! async fn get_user(
//!     token: String,
//!     id: u32,
//! ) -> ApiResult<Json<String>, (Unauthorized, NotFound<String>, Internal<String>)> {
//!     // `Unauthorized` is in the set, so `?` converts it.
//!     check_token(&token)?;
//!
//!     // `(NotFound<String>,)` is a subset of this handler's set.
//!     let name = find_user(id).into_superset()?;
//!
//!     // A `String` error -> `Internal<String>`
//!     let name = normalize(name).with_status::<Internal>()?;
//!
//!     Ok(Json(name))
//! }
//! # fn lookup(_: u32) -> Option<String> { None }
//! # fn normalize(name: String) -> Result<String, String> { Ok(name) }
//! ```
//!
//! Returning a status code that isn't in the set doesn't compile:
//! ```rust,compile_fail
//! # use axum_error_sets::{ApiResult, codes::{Forbidden, NotFound}};
//! async fn handler() -> ApiResult<(), (NotFound,)> {
//!     Err(Forbidden(()))?; // error: `(NotFound,): Contains<Forbidden>` is not satisfied
//!     Ok(())
//! }
//! ```
//!
//! More examples are in the
//! [`examples`](https://github.com/jvdwrf/axum-error-sets/tree/main/examples) directory.
//!
//! # Responses
//! When an [`ApiResponse`] is returned, the response has the status code of the wrapper it was
//! created from, even if the body's own response sets a different status. The body's
//! [`IntoResponse`] runs as soon as the `ApiResponse` is created, not when the handler
//! returns. Standard axum behaves differently here, which matters if `into_response` has side
//! effects such as logging.
//!
//! # OpenAPI with `aide`
//! With the `aide` feature enabled, `ApiResponse<S>` implements [`aide::OperationOutput`]
//! whenever every body type in `S` does. Each status code in the set is then documented as a
//! response of the operation. This works for sets of up to 16 status codes.
//!
//! # Typed routing
//! [axum-typed-routing](https://docs.rs/axum-typed-routing) is a companion crate for
//! declaring a route's path and parameters next to its handler. With its `api_route` macro,
//! the handler's error set shows up in the generated OpenAPI documentation automatically.
//!
//! # Feature flags
//! - `aide`: implements [`aide::OperationOutput`] for [`ApiResponse`].

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use type_sets::{Contains, Superset};

/// Short for `Result<T, ApiResponse<S>>`.
pub type ApiResult<T, S> = Result<T, ApiResponse<S>>;

/// An error response whose status code is one of the codes in the set `S`.
///
/// `S` is a tuple of status codes from [`codes`], for example
/// `ApiResponse<(NotFound<String>, Internal<Json<String>>)>`. Usually it is the error type of
/// a handler, written as `Result<T, ApiResponse<S>>` or [`ApiResult<T, S>`].
///
/// It implements:
/// - [`IntoResponse`], so it can be returned from axum handlers;
/// - `From<C>` for every status code `C` in `S`, so `?` converts status codes into it;
/// - `aide::OperationOutput`, when the `aide` feature is enabled and every body type in `S`
///   implements `OperationOutput`.
///
/// The body's [`IntoResponse`] runs when the `ApiResponse` is created, not when the handler
/// returns. See [Responses](crate#responses).
///
/// # Example
/// ```rust
/// # use axum_error_sets::{ApiResponse, codes::*};
/// async fn handler() -> Result<(), ApiResponse<(NotFound<String>, BadRequest<String>)>> {
///     Err(NotFound("no such item".to_string()).into())
/// }
/// ```
pub struct ApiResponse<S> {
    response: Response,
    code: StatusCode,
    _marker: std::marker::PhantomData<fn() -> S>,
}

impl<S> ApiResponse<S> {
    /// Creates an `ApiResponse` from a status code in the set `S`.
    ///
    /// Usually you don't need this, because `?` and [`Into::into`] do the same.
    pub fn new<T>(wrapper: T) -> Self
    where
        S: Contains<T>,
        T: StatusProvider<Inner: IntoResponse>,
    {
        Self::new_unchecked(wrapper.into_inner(), T::STATUS_CODE)
    }

    /// Creates an `ApiResponse` from any response and status code, without checking that the
    /// status code is in the set `S`.
    pub fn new_unchecked(response: impl IntoResponse, code: StatusCode) -> Self {
        Self {
            response: response.into_response(),
            code,
            _marker: std::marker::PhantomData,
        }
    }

    /// Splits the `ApiResponse` into the body's response and the status code.
    pub fn into_parts(self) -> (Response, StatusCode) {
        (self.response, self.code)
    }

    /// Converts into an `ApiResponse` with a larger set `U`, which must contain every status
    /// code in `S`. See also [`ApiResultExt::into_superset`].
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

        // $(
        //     #[allow(unused)]
        //     #[cfg(feature = "utoipa")]
        //     impl<$($E),*> utoipa::IntoResponses for ApiResponse<($($E,)*)>
        //     where
        //         $(
        //             $E: StatusProvider<Inner: utoipa::ToSchema>,
        //         )*
        //     {
        //         fn responses() -> std::collections::BTreeMap<
        //             String,
        //             utoipa::openapi::RefOr<utoipa::openapi::Response>,
        //         > {
        //             let mut responses = utoipa::openapi::ResponsesBuilder::new();

        //             $({
        //                 let name = < $E::Inner as utoipa::ToSchema >::name();
        //                 let schema = < $E::Inner as utoipa::PartialSchema >::schema();
        //                 let content = utoipa::openapi::ContentBuilder::new()
        //                     .schema(Some(schema))
        //                     .build();

        //                 responses = responses.response(
        //                     $E::STATUS_CODE.as_u16().to_string(),
        //                     utoipa::openapi::response::ResponseBuilder::new()
        //                         .description(format!(
        //                             "{} response for {}",
        //                             $E::STATUS_CODE.as_u16(),
        //                             name
        //                         ))
        //                         .content(
        //                             "application/json",
        //                             content
        //                         )
        //                         .build()
        //                 );

        //             })*

        //             responses.build().responses
        //         }
        //     }
        // )+
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

/// A wrapper type that pairs a body with a fixed HTTP status code.
///
/// This trait is implemented for every status code in [`codes`]. The status code is part of
/// the type, which is what lets [`ApiResponse`] track which codes a handler can return.
pub trait StatusProvider: From<Self::Inner> + Sized {
    /// The status code of this wrapper.
    const STATUS_CODE: StatusCode;

    /// The type of the wrapped body.
    type Inner;

    /// The same status code with a different body type, for example `NotFound<T>` for
    /// `NotFound<String>`.
    type WithInner<T>: StatusProvider<Inner = T>;

    /// Returns the wrapped body.
    fn into_inner(self) -> Self::Inner;

    /// Converts into an [`ApiResponse`] whose set contains this status code. This is the
    /// same as calling `into()`.
    fn into_set<E>(self) -> ApiResponse<E>
    where
        Self::Inner: IntoResponse,
        E: Contains<Self>,
    {
        ApiResponse::new(self)
    }

    /// Applies `f` to the body, keeping the status code.
    fn map<T>(self, f: impl FnOnce(Self::Inner) -> T) -> Self::WithInner<T> {
        <Self::WithInner<T> as From<T>>::from(f(self.into_inner()))
    }
}

/// Methods on any `Result` for giving its error an HTTP status code, and for changing the
/// status code or body of an error that already has one.
///
/// Each method only changes the `Err` value. `Ok` values pass through unchanged.
pub trait ResultStatusExt<T, E>: Sized {
    /// Wraps the error in the status code `S`. The error becomes the body.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use axum_error_sets::{ResultStatusExt as _, codes::BadRequest};
    /// let result: Result<(), String> = Err("error".into());
    ///
    /// // Has type `Result<(), BadRequest<String>>`
    /// let _wrapped = result.with_status::<BadRequest>();
    /// ```
    fn with_status<S>(self) -> Result<T, S::WithInner<E>>
    where
        S: StatusProvider<Inner = ()>;

    /// Like [`with_status`](ResultStatusExt::with_status), but first converts the error into
    /// `E2` with [`Into::into`].
    ///
    /// `E2` can often be inferred, as in `into_status::<BadRequest, _>()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use axum_error_sets::{ResultStatusExt as _, codes::BadRequest};
    /// let result: Result<(), &str> = Err("error");
    ///
    /// // Has type `Result<(), BadRequest<String>>`
    /// let _wrapped = result.into_status::<BadRequest, String>();
    /// ```
    fn into_status<S, E2>(self) -> Result<T, S::WithInner<E2>>
    where
        S: StatusProvider<Inner = ()>,
        E: Into<E2>;

    /// Replaces the error's status code with `S`, keeping the body.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use axum_error_sets::{ResultStatusExt as _, codes::{Forbidden, NotFound}};
    /// let result: Result<(), Forbidden<String>> = Err(Forbidden("hidden".into()));
    ///
    /// // Has type `Result<(), NotFound<String>>`
    /// let _changed = result.change_status::<NotFound>();
    /// ```
    fn change_status<S>(self) -> Result<T, S::WithInner<E::Inner>>
    where
        E: StatusProvider,
        S: StatusProvider;

    /// Applies `f` to the error's body, keeping the status code.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use axum::Json;
    /// # use axum_error_sets::{ResultStatusExt as _, codes::Internal};
    /// let result: Result<(), Internal<String>> = Err(Internal("error".into()));
    ///
    /// // Has type `Result<(), Internal<Json<String>>>`
    /// let _mapped = result.map_status(Json);
    /// ```
    fn map_status<F, O>(self, f: F) -> Result<T, E::WithInner<O>>
    where
        E: StatusProvider,
        F: FnOnce(E::Inner) -> O;

    /// Converts the error's body with [`Into::into`], keeping the status code.
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

/// Methods on `Result<T, ApiResponse<S>>`.
pub trait ApiResultExt<T, S> {
    /// Converts the error into an [`ApiResponse`] with a larger set `U`, which must contain
    /// every status code in `S`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use axum_error_sets::{ApiResult, ApiResultExt as _, codes::{Forbidden, NotFound}};
    /// fn inner() -> ApiResult<(), (NotFound,)> {
    ///     Ok(())
    /// }
    ///
    /// fn outer() -> ApiResult<(), (Forbidden, NotFound)> {
    ///     inner().into_superset()?;
    ///     Ok(())
    /// }
    /// ```
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
