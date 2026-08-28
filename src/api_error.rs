use crate::{ErrorSetValue, StatusWrapper};
use axum_core::response::{IntoResponse, Response};
use http::StatusCode;
use std::{fmt::Debug, hash::Hash, marker::PhantomData};
use type_sets::{Contains, SupersetOf};

/// An error defined by a set of possible status codes.
///
/// The parameter `T` is the actual value of the error, and must implement
/// [`ErrorSetValue`].
///
/// The parameter `E` is a type-level set of possible status codes that this error
/// can have. e.g. `(NotFound, InternalServerError)` means that this error can either
/// be a 404 or a 500.
pub struct ErrorSet<T, E> {
    value: T,
    code: StatusCode,
    _e: PhantomData<fn() -> E>,
}

impl<T, E> ErrorSet<T, E> {
    /// Create a new [`ApiError`] with the given wrapper-type.
    ///
    /// Checks at compile time that the status code is part of the set `E`.
    pub fn new_with<R: StatusWrapper>(value: T) -> Self
    where
        E: Contains<R>,
    {
        Self::new_unchecked(value, R::STATUS_CODE)
    }

    /// Create a new [`ApiError`] with the given wrapper from a type that implements
    /// [`StatusWrapper`]. (e.g. [`NotFound<YourValue>`](crate::code::NotFound))
    ///
    /// Checks at compile time that the status code is part of the set `E`.
    pub fn new<R: StatusWrapper>(value: R) -> Self
    where
        E: Contains<R::Pure>,
        R::Inner: Into<T>,
    {
        Self::new_with::<R::Pure>(value.into_inner().into())
    }

    /// Create a new [`ApiError`] with the given value and status code.
    ///
    /// This does not check that the status code is part of the set `E`. Be careful when
    /// using this method, as it can lead to invalid [`ApiError`]s.
    pub fn new_unchecked(value: T, code: StatusCode) -> Self {
        ErrorSet {
            value,
            code,
            _e: PhantomData,
        }
    }

    /// Convert this [`ApiError`] into a tuple of the value and the status code.
    pub fn into_parts(self) -> (T, StatusCode) {
        (self.value, self.code)
    }

    /// Convert this [`ApiError`] into a new [`ApiError`] with a superset of the original
    /// set of status codes.
    pub fn into_superset<E2>(self) -> ErrorSet<T, E2>
    where
        E2: SupersetOf<E>,
    {
        ErrorSet::new_unchecked(self.value, self.code)
    }

    /// Convert this [`ApiError`] into a new [`ApiError`] with a different value type.
    pub fn map_value<F, U>(self, f: F) -> ErrorSet<U, E>
    where
        F: FnOnce(T) -> U,
    {
        ErrorSet::new_unchecked(f(self.value), self.code)
    }

    /// Get a reference to the value of this [`ApiError`].
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get a mutable reference to the value of this [`ApiError`].
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Get the status code of this [`ApiError`].
    pub fn status_code(&self) -> StatusCode {
        self.code
    }
}

impl<T: ErrorSetValue, R> IntoResponse for ErrorSet<T, R> {
    fn into_response(self) -> Response {
        self.value.into_response_with(self.code)
    }
}

impl<T: Debug, R> Debug for ErrorSet<T, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiError")
            .field("value", &self.value)
            .field("code", &self.code)
            .finish()
    }
}

impl<T: Clone, R> Clone for ErrorSet<T, R> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            code: self.code,
            _e: PhantomData,
        }
    }
}

impl<T: Hash, R> Hash for ErrorSet<T, R> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.code.hash(state);
    }
}

impl<T: PartialEq, R> PartialEq for ErrorSet<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.code == other.code
    }
}

impl<T: Eq, R> Eq for ErrorSet<T, R> {}
