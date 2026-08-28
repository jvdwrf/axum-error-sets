use crate::{ApiError, code::*};
use type_sets::SupersetOf;

/// Extension trait for [`Result`] to convert it into a [`Result`] with an [`ApiError`].
pub trait StatusResultExt {
    /// The `Ok` type of the [`Result`].
    type Ok;

    /// The `Err` type of the [`Result`].
    type Err;

    /// Converts the error into a `400 Bad Request` API error.
    fn into_bad_request(self) -> Result<Self::Ok, BadRequest<Self::Err>>;

    /// Converts the error into a `401 Unauthorized` API error.
    fn into_unauthorized(self) -> Result<Self::Ok, Unauthorized<Self::Err>>;

    /// Converts the error into a `403 Forbidden` API error.
    fn into_forbidden(self) -> Result<Self::Ok, Forbidden<Self::Err>>;

    /// Converts the error into a `404 Not Found` API error.
    fn into_not_found(self) -> Result<Self::Ok, NotFound<Self::Err>>;

    /// Converts the error into a `409 Conflict` API error.
    fn into_conflict(self) -> Result<Self::Ok, Conflict<Self::Err>>;

    /// Converts the error into a `422 Unprocessable Entity` API error.
    fn into_unprocessable_entity(self) -> Result<Self::Ok, UnprocessableEntity<Self::Err>>;

    /// Converts the error into a `429 Too Many Requests` API error.
    fn into_too_many_requests(self) -> Result<Self::Ok, TooManyRequests<Self::Err>>;

    /// Converts the error into a `500 Internal Server Error` API error.
    fn into_internal(self) -> Result<Self::Ok, InternalServerError<Self::Err>>;

    /// Converts the error into a `502 Bad Gateway` API error.
    fn into_bad_gateway(self) -> Result<Self::Ok, BadGateway<Self::Err>>;

    /// Converts the error into a `503 Service Unavailable` API error.
    fn into_service_unavailable(self) -> Result<Self::Ok, ServiceUnavailable<Self::Err>>;

    /// Converts the error into a `504 Gateway Timeout` API error.
    fn into_gateway_timeout(self) -> Result<Self::Ok, GatewayTimeout<Self::Err>>;
}

impl<T, E> StatusResultExt for Result<T, E> {
    type Ok = T;
    type Err = E;

    fn into_bad_request(self) -> Result<T, BadRequest<E>> {
        self.map_err(BadRequest)
    }

    fn into_unauthorized(self) -> Result<T, Unauthorized<E>> {
        self.map_err(Unauthorized)
    }

    fn into_forbidden(self) -> Result<T, Forbidden<E>> {
        self.map_err(Forbidden)
    }

    fn into_not_found(self) -> Result<T, NotFound<E>> {
        self.map_err(NotFound)
    }

    fn into_conflict(self) -> Result<T, Conflict<E>> {
        self.map_err(Conflict)
    }

    fn into_unprocessable_entity(self) -> Result<T, UnprocessableEntity<E>> {
        self.map_err(UnprocessableEntity)
    }

    fn into_too_many_requests(self) -> Result<T, TooManyRequests<E>> {
        self.map_err(TooManyRequests)
    }

    fn into_internal(self) -> Result<T, InternalServerError<E>> {
        self.map_err(InternalServerError)
    }

    fn into_bad_gateway(self) -> Result<T, BadGateway<E>> {
        self.map_err(BadGateway)
    }

    fn into_service_unavailable(self) -> Result<T, ServiceUnavailable<E>> {
        self.map_err(ServiceUnavailable)
    }

    fn into_gateway_timeout(self) -> Result<T, GatewayTimeout<E>> {
        self.map_err(GatewayTimeout)
    }
}

pub trait ApiResultExt {
    type Ok;
    type Err;
    type Value;

    fn into_superset<E>(self) -> Result<Self::Ok, ApiError<Self::Value, E>>
    where
        E: SupersetOf<Self::Err>;
}

impl<T, E, V> ApiResultExt for Result<T, ApiError<V, E>> {
    type Ok = T;
    type Err = E;
    type Value = V;

    fn into_superset<E2>(self) -> Result<T, ApiError<V, E2>>
    where
        E2: SupersetOf<E>,
    {
        self.map_err(|e| e.into_superset())
    }
}
