//! Defines all HTTP status code wrappers.

use super::*;

macro_rules! define_codes {
(
    $(
        $(#[$meta:meta])*
        $(fn $fn_name:ident ();)?
        code $name:ident => $status:expr;
    )*
) => {
    $(
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $name<T = ()>(pub T);

        impl<T> StatusProvider for $name<T> {
            const STATUS_CODE: StatusCode = $status;
            type Inner = T;
            type WithInner<R> = $name<R>;
            fn into_inner(self) -> Self::Inner {
                self.0
            }
        }

        impl<T, S> From<$name<T>> for ApiResponse<S>
        where
            S: Contains<$name<T>>,
            $name<T>: StatusProvider<Inner: IntoResponse>,
        {
            fn from(err: $name<T>) -> Self {
                ApiResponse::new(err)
            }
        }

        impl<T> From<T> for $name<T> {
            fn from(inner: T) -> Self {
                Self(inner)
            }
        }

        impl<T> std::ops::Deref for $name<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for $name<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    )*
};
}

define_codes!(
    /// 400 Bad Request
    fn map_bad_request();
    code BadRequest => StatusCode::BAD_REQUEST;

    /// 401 Unauthorized
    fn map_unauthorized();
    code Unauthorized => StatusCode::UNAUTHORIZED;

    /// 402 Payment Required
    fn map_payment_required();
    code PaymentRequired => StatusCode::PAYMENT_REQUIRED;

    /// 403 Forbidden
    fn map_forbidden();
    code Forbidden => StatusCode::FORBIDDEN;

    /// 404 Not Found
    fn map_not_found();
    code NotFound => StatusCode::NOT_FOUND;

    /// 405 Method Not Allowed
    fn map_method_not_allowed();
    code MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED;

    /// 406 Not Acceptable
    fn map_not_acceptable();
    code NotAcceptable => StatusCode::NOT_ACCEPTABLE;

    /// 407 Proxy Authentication Required
    fn map_proxy_authentication_required();
    code ProxyAuthenticationRequired => StatusCode::PROXY_AUTHENTICATION_REQUIRED;

    /// 408 Request Timeout
    fn map_request_timeout();
    code RequestTimeout => StatusCode::REQUEST_TIMEOUT;

    /// 409 Conflict
    fn map_conflict();
    code Conflict => StatusCode::CONFLICT;

    /// 410 Gone
    fn map_gone();
    code Gone => StatusCode::GONE;

    /// 411 Length Required
    fn map_length_required();
    code LengthRequired => StatusCode::LENGTH_REQUIRED;

    /// 412 Precondition Failed
    fn map_precondition_failed();
    code PreconditionFailed => StatusCode::PRECONDITION_FAILED;

    /// 413 Payload Too Large
    fn map_payload_too_large();
    code PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE;

    /// 414 URI Too Long
    fn map_uri_too_long();
    code UriTooLong => StatusCode::URI_TOO_LONG;

    /// 415 Unsupported Media Type
    fn map_unsupported_media_type();
    code UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE;

    /// 416 Range Not Satisfiable
    fn map_range_not_satisfiable();
    code RangeNotSatisfiable => StatusCode::RANGE_NOT_SATISFIABLE;

    /// 417 Expectation Failed
    fn map_expectation_failed();
    code ExpectationFailed => StatusCode::EXPECTATION_FAILED;

    /// 418 I'm a teapot
    fn map_im_a_teapot();
    code ImATeapot => StatusCode::IM_A_TEAPOT;

    /// 421 Misdirected Request
    fn map_misdirected_request();
    code MisdirectedRequest => StatusCode::MISDIRECTED_REQUEST;

    /// 422 Unprocessable Entity
    fn map_unprocessable_entity();
    code UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY;

    /// 423 Locked
    fn map_locked();
    code Locked => StatusCode::LOCKED;

    /// 424 Failed Dependency
    fn map_failed_dependency();
    code FailedDependency => StatusCode::FAILED_DEPENDENCY;

    /// 425 Too Early
    fn map_too_early();
    code TooEarly => StatusCode::TOO_EARLY;

    /// 426 Upgrade Required
    fn map_upgrade_required();
    code UpgradeRequired => StatusCode::UPGRADE_REQUIRED;

    /// 428 Precondition Required
    fn map_precondition_required();
    code PreconditionRequired => StatusCode::PRECONDITION_REQUIRED;

    /// 429 Too Many Requests
    fn map_too_many_requests();
    code TooManyRequests => StatusCode::TOO_MANY_REQUESTS;

    /// 431 Request Header Fields Too Large
    fn map_request_header_fields_too_large();
    code RequestHeaderFieldsTooLarge => StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE;

    /// 451 Unavailable For Legal Reasons
    fn map_unavailable_for_legal_reasons();
    code UnavailableForLegalReasons => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS;

    /// 500 Internal Server Error
    fn map_internal();
    code Internal => StatusCode::INTERNAL_SERVER_ERROR;

    /// 501 Not Implemented
    fn map_not_implemented();
    code NotImplemented => StatusCode::NOT_IMPLEMENTED;

    /// 502 Bad Gateway
    fn map_bad_gateway();
    code BadGateway => StatusCode::BAD_GATEWAY;

    /// 503 Service Unavailable
    fn map_service_unavailable();
    code ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE;

    /// 504 Gateway Timeout
    fn map_gateway_timeout();
    code GatewayTimeout => StatusCode::GATEWAY_TIMEOUT;

    /// 505 HTTP Version Not Supported
    fn map_http_version_not_supported();
    code HttpVersionNotSupported => StatusCode::HTTP_VERSION_NOT_SUPPORTED;

    /// 506 Variant Also Negotiates
    fn map_variant_also_negotiates();
    code VariantAlsoNegotiates => StatusCode::VARIANT_ALSO_NEGOTIATES;

    /// 507 Insufficient Storage
    fn map_insufficient_storage();
    code InsufficientStorage => StatusCode::INSUFFICIENT_STORAGE;

    /// 508 Loop Detected
    fn map_loop_detected();
    code LoopDetected => StatusCode::LOOP_DETECTED;

    /// 510 Not Extended
    fn map_not_extended();
    code NotExtended => StatusCode::NOT_EXTENDED;

    /// 511 Network Authentication Required
    fn map_network_authentication_required();
    code NetworkAuthenticationRequired => StatusCode::NETWORK_AUTHENTICATION_REQUIRED;
);
