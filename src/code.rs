use super::*;

macro_rules! define_codes {
    (
        $(
            $(#[$meta:meta])*
            code $name:ident => $status:expr;
        )*
    ) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy)]
            pub struct $name<T = ()>(pub T);

            impl<T, R, S> From<$name<R>> for ErrorSet<S, T>
            where
                R: Into<S>,
                T: Contains<$name>,
            {
                fn from(err: $name<R>) -> Self {
                    ErrorSet::new(err)
                }
            }

            impl<T> StatusWrapper for $name<T> {
                const STATUS_CODE: StatusCode = $status;

                type Inner = T;
                type Pure = $name;

                fn into_inner(self) -> Self::Inner {
                    self.0
                }
            }
        )*
    };
}

define_codes!(
    /// 400 Bad Request
    code BadRequest => StatusCode::BAD_REQUEST;

    /// 401 Unauthorized
    code Unauthorized => StatusCode::UNAUTHORIZED;

    /// 402 Payment Required
    code PaymentRequired => StatusCode::PAYMENT_REQUIRED;

    /// 403 Forbidden
    code Forbidden => StatusCode::FORBIDDEN;

    /// 404 Not Found
    code NotFound => StatusCode::NOT_FOUND;

    /// 405 Method Not Allowed
    code MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED;

    /// 406 Not Acceptable
    code NotAcceptable => StatusCode::NOT_ACCEPTABLE;

    /// 407 Proxy Authentication Required
    code ProxyAuthenticationRequired => StatusCode::PROXY_AUTHENTICATION_REQUIRED;

    /// 408 Request Timeout
    code RequestTimeout => StatusCode::REQUEST_TIMEOUT;

    /// 409 Conflict
    code Conflict => StatusCode::CONFLICT;

    /// 410 Gone
    code Gone => StatusCode::GONE;

    /// 411 Length Required
    code LengthRequired => StatusCode::LENGTH_REQUIRED;

    /// 412 Precondition Failed
    code PreconditionFailed => StatusCode::PRECONDITION_FAILED;

    // /// 413 Content Too Large
    // code ContentTooLarge => StatusCode::CONTENT_TOO_LARGE;

    /// 414 URI Too Long
    code UriTooLong => StatusCode::URI_TOO_LONG;

    /// 415 Unsupported Media Type
    code UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE;

    /// 416 Range Not Satisfiable
    code RangeNotSatisfiable => StatusCode::RANGE_NOT_SATISFIABLE;

    /// 417 Expectation Failed
    code ExpectationFailed => StatusCode::EXPECTATION_FAILED;

    /// 418 I'm a teapot
    code ImATeapot => StatusCode::IM_A_TEAPOT;

    /// 421 Misdirected Request
    code MisdirectedRequest => StatusCode::MISDIRECTED_REQUEST;

    /// 422 Unprocessable Entity
    code UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY;

    /// 423 Locked
    code Locked => StatusCode::LOCKED;

    /// 424 Failed Dependency
    code FailedDependency => StatusCode::FAILED_DEPENDENCY;

    // /// 425 Too Early
    // code TooEarly => StatusCode::TOO_EARLY;

    /// 426 Upgrade Required
    code UpgradeRequired => StatusCode::UPGRADE_REQUIRED;

    /// 428 Precondition Required
    code PreconditionRequired => StatusCode::PRECONDITION_REQUIRED;

    /// 429 Too Many Requests
    code TooManyRequests => StatusCode::TOO_MANY_REQUESTS;

    /// 431 Request Header Fields Too Large
    code RequestHeaderFieldsTooLarge => StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE;

    /// 451 Unavailable For Legal Reasons
    code UnavailableForLegalReasons => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS;

    /// 500 Internal Server Error
    code InternalServerError => StatusCode::INTERNAL_SERVER_ERROR;

    /// 501 Not Implemented
    code NotImplemented => StatusCode::NOT_IMPLEMENTED;

    /// 502 Bad Gateway
    code BadGateway => StatusCode::BAD_GATEWAY;

    /// 503 Service Unavailable
    code ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE;

    /// 504 Gateway Timeout
    code GatewayTimeout => StatusCode::GATEWAY_TIMEOUT;

    /// 505 HTTP Version Not Supported
    code HttpVersionNotSupported => StatusCode::HTTP_VERSION_NOT_SUPPORTED;

    /// 506 Variant Also Negotiates
    code VariantAlsoNegotiates => StatusCode::VARIANT_ALSO_NEGOTIATES;

    /// 507 Insufficient Storage
    code InsufficientStorage => StatusCode::INSUFFICIENT_STORAGE;

    /// 508 Loop Detected
    code LoopDetected => StatusCode::LOOP_DETECTED;

    /// 510 Not Extended
    code NotExtended => StatusCode::NOT_EXTENDED;

    /// 511 Network Authentication Required
    code NetworkAuthenticationRequired => StatusCode::NETWORK_AUTHENTICATION_REQUIRED;
);
