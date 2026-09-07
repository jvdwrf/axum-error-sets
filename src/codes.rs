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

        impl<T> WrapsResponse for $name<T> {
            const STATUS_CODE: StatusCode = $status;
            type Inner = T;
            type Pure = $name;
            fn into_inner(self) -> Self::Inner {
                self.0
            }
        }

        impl<T, S> From<$name<T>> for ApiResponse<S>
        where
            S: Contains<$name<T>>,
            $name<T>: WrapsResponse<Inner: IntoResponse>,
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

    pub trait ApiResultExt<T, E> {
        $(
            $(
                fn $fn_name<R>(self) -> Result<T, $name<R>>
                where
                    E: Into<R>;
            )?
        )*
    }

    impl<T, E> ApiResultExt<T, E> for Result<T, E> {
        $(
            $(
                fn $fn_name<R>(self) -> Result<T, $name<R>>
                where
                    E: Into<R>,
                {
                    match self {
                        Ok(val) => Ok(val),
                        Err(e) => Err($name(e.into())),
                    }
                }
            )?
        )*
    }
};
}

define_codes!(
    /// 400 Bad Request
    fn bad_request();
    code BadRequest => StatusCode::BAD_REQUEST;

    /// 401 Unauthorized
    fn unauthorized_err();
    code Unauthorized => StatusCode::UNAUTHORIZED;

    /// 402 Payment Required
    fn payment_required_err();
    code PaymentRequired => StatusCode::PAYMENT_REQUIRED;

    /// 403 Forbidden
    fn forbidden_err();
    code Forbidden => StatusCode::FORBIDDEN;

    /// 404 Not Found
    fn not_found_err();
    code NotFound => StatusCode::NOT_FOUND;

    /// 405 Method Not Allowed
    fn method_not_allowed_err();
    code MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED;

    /// 406 Not Acceptable
    fn not_acceptable_err();
    code NotAcceptable => StatusCode::NOT_ACCEPTABLE;

    /// 407 Proxy Authentication Required
    fn proxy_authentication_required_err();
    code ProxyAuthenticationRequired => StatusCode::PROXY_AUTHENTICATION_REQUIRED;

    /// 408 Request Timeout
    fn request_timeout_err();
    code RequestTimeout => StatusCode::REQUEST_TIMEOUT;

    /// 409 Conflict
    fn conflict_err();
    code Conflict => StatusCode::CONFLICT;

    /// 410 Gone
    fn gone_err();
    code Gone => StatusCode::GONE;

    /// 411 Length Required
    fn length_required_err();
    code LengthRequired => StatusCode::LENGTH_REQUIRED;

    /// 412 Precondition Failed
    fn precondition_failed_err();
    code PreconditionFailed => StatusCode::PRECONDITION_FAILED;

    // /// 413 Content Too Large
    // code ContentTooLarge => StatusCode::CONTENT_TOO_LARGE;

    /// 414 URI Too Long
    fn uri_too_long_err();
    code UriTooLong => StatusCode::URI_TOO_LONG;

    /// 415 Unsupported Media Type
    fn unsupported_media_type_err();
    code UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE;

    /// 416 Range Not Satisfiable
    fn range_not_satisfiable_err();
    code RangeNotSatisfiable => StatusCode::RANGE_NOT_SATISFIABLE;

    /// 417 Expectation Failed
    fn expectation_failed_err();
    code ExpectationFailed => StatusCode::EXPECTATION_FAILED;

    /// 418 I'm a teapot
    fn im_a_teapot_err();
    code ImATeapot => StatusCode::IM_A_TEAPOT;

    /// 421 Misdirected Request
    fn misdirected_request_err();
    code MisdirectedRequest => StatusCode::MISDIRECTED_REQUEST;

    /// 422 Unprocessable Entity
    fn unprocessable_entity_err();
    code UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY;

    /// 423 Locked
    fn locked_err();
    code Locked => StatusCode::LOCKED;

    /// 424 Failed Dependency
    fn failed_dependency_err();
    code FailedDependency => StatusCode::FAILED_DEPENDENCY;

    // /// 425 Too Early
    // code TooEarly => StatusCode::TOO_EARLY;

    /// 426 Upgrade Required
    fn upgrade_required_err();
    code UpgradeRequired => StatusCode::UPGRADE_REQUIRED;

    /// 428 Precondition Required
    fn precondition_required_err();
    code PreconditionRequired => StatusCode::PRECONDITION_REQUIRED;

    /// 429 Too Many Requests
    fn too_many_requests_err();
    code TooManyRequests => StatusCode::TOO_MANY_REQUESTS;

    /// 431 Request Header Fields Too Large
    fn request_header_fields_too_large_err();
    code RequestHeaderFieldsTooLarge => StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE;

    /// 451 Unavailable For Legal Reasons
    fn unavailable_for_legal_reasons_err();
    code UnavailableForLegalReasons => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS;

    /// 500 Internal Server Error
    fn internal_err();
    code InternalServerError => StatusCode::INTERNAL_SERVER_ERROR;

    /// 501 Not Implemented
    fn not_implemented_err();
    code NotImplemented => StatusCode::NOT_IMPLEMENTED;

    /// 502 Bad Gateway
    fn bad_gateway_err();
    code BadGateway => StatusCode::BAD_GATEWAY;

    /// 503 Service Unavailable
    fn service_unavailable_err();
    code ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE;

    /// 504 Gateway Timeout
    fn gateway_timeout_err();
    code GatewayTimeout => StatusCode::GATEWAY_TIMEOUT;

    /// 505 HTTP Version Not Supported
    fn http_version_not_supported_err();
    code HttpVersionNotSupported => StatusCode::HTTP_VERSION_NOT_SUPPORTED;

    /// 506 Variant Also Negotiates
    fn variant_also_negotiates_err();
    code VariantAlsoNegotiates => StatusCode::VARIANT_ALSO_NEGOTIATES;

    /// 507 Insufficient Storage
    fn insufficient_storage_err();
    code InsufficientStorage => StatusCode::INSUFFICIENT_STORAGE;

    /// 508 Loop Detected
    fn loop_detected_err();
    code LoopDetected => StatusCode::LOOP_DETECTED;

    /// 510 Not Extended
    fn not_extended_err();
    code NotExtended => StatusCode::NOT_EXTENDED;

    /// 511 Network Authentication Required
    fn network_authentication_required_err();
    code NetworkAuthenticationRequired => StatusCode::NETWORK_AUTHENTICATION_REQUIRED;
);
