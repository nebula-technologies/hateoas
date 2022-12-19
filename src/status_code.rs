use std::fmt;
use std::fmt::Formatter;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub enum StatusCode {
    /// 100 Continue
    /// [[RFC7231, Section 6.2.1](https://tools.ietf.org/html/rfc7231#section-6.2.1)]
    Continue,
    /// 101 Switching Protocols
    /// [[RFC7231, Section 6.2.2](https://tools.ietf.org/html/rfc7231#section-6.2.2)]
    SwitchingProtocols,
    /// 102 Processing
    /// [[RFC2518](https://tools.ietf.org/html/rfc2518)]
    Processing,
    /// 200 OK
    /// [[RFC7231, Section 6.3.1](https://tools.ietf.org/html/rfc7231#section-6.3.1)]
    OK,
    /// 201 Created
    /// [[RFC7231, Section 6.3.2](https://tools.ietf.org/html/rfc7231#section-6.3.2)]
    Created,
    /// 202 Accepted
    /// [[RFC7231, Section 6.3.3](https://tools.ietf.org/html/rfc7231#section-6.3.3)]
    Accepted,
    /// 203 Non-Authoritative Information
    /// [[RFC7231, Section 6.3.4](https://tools.ietf.org/html/rfc7231#section-6.3.4)]
    NonAuthoritativeInformation,
    /// 204 No Content
    /// [[RFC7231, Section 6.3.5](https://tools.ietf.org/html/rfc7231#section-6.3.5)]
    NoContent,
    /// 205 Reset Content
    /// [[RFC7231, Section 6.3.6](https://tools.ietf.org/html/rfc7231#section-6.3.6)]
    ResetContent,
    /// 206 Partial Content
    /// [[RFC7233, Section 4.1](https://tools.ietf.org/html/rfc7233#section-4.1)]
    PartialContent,
    /// 207 Multi-Status
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    MultiStatus,
    /// 208 Already Reported
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    AlreadyReported,
    /// 226 IM Used
    /// [[RFC3229](https://tools.ietf.org/html/rfc3229)]
    IMUsed,
    /// 300 Multiple Choices
    /// [[RFC7231, Section 6.4.1](https://tools.ietf.org/html/rfc7231#section-6.4.1)]
    MultipleChoices,
    /// 301 Moved Permanently
    /// [[RFC7231, Section 6.4.2](https://tools.ietf.org/html/rfc7231#section-6.4.2)]
    MovedPermanently,
    /// 302 Found
    /// [[RFC7231, Section 6.4.3](https://tools.ietf.org/html/rfc7231#section-6.4.3)]
    Found,
    /// 303 See Other
    /// [[RFC7231, Section 6.4.4](https://tools.ietf.org/html/rfc7231#section-6.4.4)]
    SeeOther,
    /// 304 Not Modified
    /// [[RFC7232, Section 4.1](https://tools.ietf.org/html/rfc7232#section-4.1)]
    NotModified,
    /// 305 Use Proxy
    /// [[RFC7231, Section 6.4.5](https://tools.ietf.org/html/rfc7231#section-6.4.5)]
    UseProxy,
    /// 307 Temporary Redirect
    /// [[RFC7231, Section 6.4.7](https://tools.ietf.org/html/rfc7231#section-6.4.7)]
    TemporaryRedirect,
    /// 308 Permanent Redirect
    /// [[RFC7238](https://tools.ietf.org/html/rfc7238)]
    PermanentRedirect,
    /// 400 Bad Request
    /// [[RFC7231, Section 6.5.1](https://tools.ietf.org/html/rfc7231#section-6.5.1)]
    BadRequest,
    /// 401 Unauthorized
    /// [[RFC7235, Section 3.1](https://tools.ietf.org/html/rfc7235#section-3.1)]
    Unauthorized,
    /// 402 Payment Required
    /// [[RFC7231, Section 6.5.2](https://tools.ietf.org/html/rfc7231#section-6.5.2)]
    PaymentRequired,
    /// 403 Forbidden
    /// [[RFC7231, Section 6.5.3](https://tools.ietf.org/html/rfc7231#section-6.5.3)]
    Forbidden,
    /// 404 Not Found
    /// [[RFC7231, Section 6.5.4](https://tools.ietf.org/html/rfc7231#section-6.5.4)]
    NotFound,
    /// 405 Method Not Allowed
    /// [[RFC7231, Section 6.5.5](https://tools.ietf.org/html/rfc7231#section-6.5.5)]
    MethodNotAllowed,
    /// 406 Not Acceptable
    /// [[RFC7231, Section 6.5.6](https://tools.ietf.org/html/rfc7231#section-6.5.6)]
    NotAcceptable,
    /// 407 Proxy Authentication Required
    /// [[RFC7235, Section 3.2](https://tools.ietf.org/html/rfc7235#section-3.2)]
    ProxyAuthenticationRequired,
    /// 408 Request Timeout
    /// [[RFC7231, Section 6.5.7](https://tools.ietf.org/html/rfc7231#section-6.5.7)]
    RequestTimeout,
    /// 409 Conflict
    /// [[RFC7231, Section 6.5.8](https://tools.ietf.org/html/rfc7231#section-6.5.8)]
    Conflict,
    /// 410 Gone
    /// [[RFC7231, Section 6.5.9](https://tools.ietf.org/html/rfc7231#section-6.5.9)]
    Gone,
    /// 411 Length Required
    /// [[RFC7231, Section 6.5.10](https://tools.ietf.org/html/rfc7231#section-6.5.10)]
    LengthRequired,
    /// 412 Precondition Failed
    /// [[RFC7232, Section 4.2](https://tools.ietf.org/html/rfc7232#section-4.2)]
    PreconditionFailed,
    /// 413 Payload Too Large
    /// [[RFC7231, Section 6.5.11](https://tools.ietf.org/html/rfc7231#section-6.5.11)]
    PayloadTooLarge,
    /// 414 URI Too Long
    /// [[RFC7231, Section 6.5.12](https://tools.ietf.org/html/rfc7231#section-6.5.12)]
    URITooLong,
    /// 415 Unsupported Media Type
    /// [[RFC7231, Section 6.5.13](https://tools.ietf.org/html/rfc7231#section-6.5.13)]
    UnsupportedMediaType,
    /// 416 Range Not Satisfiable
    /// [[RFC7233, Section 4.4](https://tools.ietf.org/html/rfc7233#section-4.4)]
    RangeNotSatisfiable,
    /// 417 Expectation Failed
    /// [[RFC7231, Section 6.5.14](https://tools.ietf.org/html/rfc7231#section-6.5.14)]
    ExpectationFailed,
    /// 418 I'm a teapot
    /// [curiously not registered by IANA but [RFC2324](https://tools.ietf.org/html/rfc2324)]
    IAmATeapot,
    /// 421 Misdirected Request
    /// [RFC7540, Section 9.1.2](http://tools.ietf.org/html/rfc7540#section-9.1.2)
    MisdirectedRequest,
    /// 422 Unprocessable Entity
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    UnprocessableEntity,
    /// 423 Locked
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    Locked,
    /// 424 Failed Dependency
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    FailedDependency,
    /// 426 Upgrade Required
    /// [[RFC7231, Section 6.5.15](https://tools.ietf.org/html/rfc7231#section-6.5.15)]
    UpgradeRequired,
    /// 428 Precondition Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    PreconditionRequired,
    /// 429 Too Many Requests
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    TooManyRequests,
    /// 431 Request Header Fields Too Large
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    RequestHeaderFieldsTooLarge,
    /// 451 Unavailable For Legal Reasons
    /// [[RFC7725](http://tools.ietf.org/html/rfc7725)]
    UnavailableForLegalReasons,
    /// 500 Internal Server Error
    /// [[RFC7231, Section 6.6.1](https://tools.ietf.org/html/rfc7231#section-6.6.1)]
    InternalServerError,
    /// 501 Not Implemented
    /// [[RFC7231, Section 6.6.2](https://tools.ietf.org/html/rfc7231#section-6.6.2)]
    NotImplemented,
    /// 502 Bad Gateway
    /// [[RFC7231, Section 6.6.3](https://tools.ietf.org/html/rfc7231#section-6.6.3)]
    BadGateway,
    /// 503 Service Unavailable
    /// [[RFC7231, Section 6.6.4](https://tools.ietf.org/html/rfc7231#section-6.6.4)]
    ServiceUnavailable,
    /// 504 Gateway Timeout
    /// [[RFC7231, Section 6.6.5](https://tools.ietf.org/html/rfc7231#section-6.6.5)]
    GatewayTimeout,
    /// 505 HTTP Version Not Supported
    /// [[RFC7231, Section 6.6.6](https://tools.ietf.org/html/rfc7231#section-6.6.6)]
    HTTPVersionNotSupported,
    /// 506 Variant Also Negotiates
    /// [[RFC2295](https://tools.ietf.org/html/rfc2295)]
    VariantAlsoNegotiates,
    /// 507 Insufficient Storage
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    InsufficientStorage,
    /// 508 Loop Detected
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    LoopDetected,
    /// 510 Not Extended
    /// [[RFC2774](https://tools.ietf.org/html/rfc2774)]
    NotExtended,
    /// 511 Network Authentication Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    NetworkAuthenticationRequired,
    /// Custom status code
    Custom(u16),
}

impl From<u16> for StatusCode {
    fn from(t: u16) -> Self {
        Self::from(&t)
    }
}
impl From<&u16> for StatusCode {
    fn from(t: &u16) -> Self {
        match t {
            100 => Self::Continue,
            101 => Self::SwitchingProtocols,
            102 => Self::Processing,
            200 => Self::OK,
            201 => Self::Created,
            202 => Self::Accepted,
            203 => Self::NonAuthoritativeInformation,
            204 => Self::NoContent,
            205 => Self::ResetContent,
            206 => Self::PartialContent,
            207 => Self::MultiStatus,
            208 => Self::AlreadyReported,
            226 => Self::IMUsed,
            300 => Self::MultipleChoices,
            301 => Self::MovedPermanently,
            302 => Self::Found,
            303 => Self::SeeOther,
            304 => Self::NotModified,
            305 => Self::UseProxy,
            307 => Self::TemporaryRedirect,
            308 => Self::PermanentRedirect,
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            402 => Self::PaymentRequired,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            406 => Self::NotAcceptable,
            407 => Self::ProxyAuthenticationRequired,
            408 => Self::RequestTimeout,
            409 => Self::Conflict,
            410 => Self::Gone,
            411 => Self::LengthRequired,
            412 => Self::PreconditionFailed,
            413 => Self::PayloadTooLarge,
            414 => Self::URITooLong,
            415 => Self::UnsupportedMediaType,
            416 => Self::RangeNotSatisfiable,
            417 => Self::ExpectationFailed,
            418 => Self::IAmATeapot,
            421 => Self::MisdirectedRequest,
            422 => Self::UnprocessableEntity,
            423 => Self::Locked,
            424 => Self::FailedDependency,
            426 => Self::UpgradeRequired,
            428 => Self::PreconditionRequired,
            429 => Self::TooManyRequests,
            431 => Self::RequestHeaderFieldsTooLarge,
            451 => Self::UnavailableForLegalReasons,
            500 => Self::InternalServerError,
            501 => Self::NotImplemented,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            505 => Self::HTTPVersionNotSupported,
            506 => Self::VariantAlsoNegotiates,
            507 => Self::InsufficientStorage,
            508 => Self::LoopDetected,
            510 => Self::NotExtended,
            511 => Self::NetworkAuthenticationRequired,
            _ => Self::Custom(*t),
        }
    }
}

impl From<StatusCode> for u16 {
    fn from(t: StatusCode) -> Self {
        Self::from(&t)
    }
}
impl From<&StatusCode> for u16 {
    fn from(t: &StatusCode) -> Self {
        match t {
            StatusCode::Continue => 100,
            StatusCode::SwitchingProtocols => 101,
            StatusCode::Processing => 102,
            StatusCode::OK => 200,
            StatusCode::Created => 201,
            StatusCode::Accepted => 202,
            StatusCode::NonAuthoritativeInformation => 203,
            StatusCode::NoContent => 204,
            StatusCode::ResetContent => 205,
            StatusCode::PartialContent => 206,
            StatusCode::MultiStatus => 207,
            StatusCode::AlreadyReported => 208,
            StatusCode::IMUsed => 226,
            StatusCode::MultipleChoices => 300,
            StatusCode::MovedPermanently => 301,
            StatusCode::Found => 302,
            StatusCode::SeeOther => 303,
            StatusCode::NotModified => 304,
            StatusCode::UseProxy => 305,
            StatusCode::TemporaryRedirect => 307,
            StatusCode::PermanentRedirect => 308,
            StatusCode::BadRequest => 400,
            StatusCode::Unauthorized => 401,
            StatusCode::PaymentRequired => 402,
            StatusCode::Forbidden => 403,
            StatusCode::NotFound => 404,
            StatusCode::MethodNotAllowed => 405,
            StatusCode::NotAcceptable => 406,
            StatusCode::ProxyAuthenticationRequired => 407,
            StatusCode::RequestTimeout => 408,
            StatusCode::Conflict => 409,
            StatusCode::Gone => 410,
            StatusCode::LengthRequired => 411,
            StatusCode::PreconditionFailed => 412,
            StatusCode::PayloadTooLarge => 413,
            StatusCode::URITooLong => 414,
            StatusCode::UnsupportedMediaType => 415,
            StatusCode::RangeNotSatisfiable => 416,
            StatusCode::ExpectationFailed => 417,
            StatusCode::IAmATeapot => 418,
            StatusCode::MisdirectedRequest => 421,
            StatusCode::UnprocessableEntity => 422,
            StatusCode::Locked => 423,
            StatusCode::FailedDependency => 424,
            StatusCode::UpgradeRequired => 426,
            StatusCode::PreconditionRequired => 428,
            StatusCode::TooManyRequests => 429,
            StatusCode::RequestHeaderFieldsTooLarge => 431,
            StatusCode::UnavailableForLegalReasons => 451,
            StatusCode::InternalServerError => 500,
            StatusCode::NotImplemented => 501,
            StatusCode::BadGateway => 502,
            StatusCode::ServiceUnavailable => 503,
            StatusCode::GatewayTimeout => 504,
            StatusCode::HTTPVersionNotSupported => 505,
            StatusCode::VariantAlsoNegotiates => 506,
            StatusCode::InsufficientStorage => 507,
            StatusCode::LoopDetected => 508,
            StatusCode::NotExtended => 510,
            StatusCode::NetworkAuthenticationRequired => 511,
            StatusCode::Custom(t) => *t,
        }
    }
}

impl From<StatusCode> for http::StatusCode {
    fn from(t: StatusCode) -> Self {
        match t {
            StatusCode::Continue => Self::CONTINUE,
            StatusCode::SwitchingProtocols => Self::SWITCHING_PROTOCOLS,
            StatusCode::Processing => Self::PROCESSING,
            StatusCode::OK => Self::OK,
            StatusCode::Created => Self::CREATED,
            StatusCode::Accepted => Self::ACCEPTED,
            StatusCode::NonAuthoritativeInformation => Self::NON_AUTHORITATIVE_INFORMATION,
            StatusCode::NoContent => Self::NO_CONTENT,
            StatusCode::ResetContent => Self::RESET_CONTENT,
            StatusCode::PartialContent => Self::PARTIAL_CONTENT,
            StatusCode::MultiStatus => Self::MULTI_STATUS,
            StatusCode::AlreadyReported => Self::ALREADY_REPORTED,
            StatusCode::IMUsed => Self::IM_USED,
            StatusCode::MultipleChoices => Self::MULTIPLE_CHOICES,
            StatusCode::MovedPermanently => Self::MOVED_PERMANENTLY,
            StatusCode::Found => Self::FOUND,
            StatusCode::SeeOther => Self::SEE_OTHER,
            StatusCode::NotModified => Self::NOT_MODIFIED,
            StatusCode::UseProxy => Self::USE_PROXY,
            StatusCode::TemporaryRedirect => Self::TEMPORARY_REDIRECT,
            StatusCode::PermanentRedirect => Self::PERMANENT_REDIRECT,
            StatusCode::BadRequest => Self::BAD_REQUEST,
            StatusCode::Unauthorized => Self::UNAUTHORIZED,
            StatusCode::PaymentRequired => Self::PAYMENT_REQUIRED,
            StatusCode::Forbidden => Self::FORBIDDEN,
            StatusCode::NotFound => Self::NOT_FOUND,
            StatusCode::MethodNotAllowed => Self::METHOD_NOT_ALLOWED,
            StatusCode::NotAcceptable => Self::NOT_ACCEPTABLE,
            StatusCode::ProxyAuthenticationRequired => Self::PROXY_AUTHENTICATION_REQUIRED,
            StatusCode::RequestTimeout => Self::REQUEST_TIMEOUT,
            StatusCode::Conflict => Self::CONFLICT,
            StatusCode::Gone => Self::GONE,
            StatusCode::LengthRequired => Self::LENGTH_REQUIRED,
            StatusCode::PreconditionFailed => Self::PRECONDITION_FAILED,
            StatusCode::PayloadTooLarge => Self::PAYLOAD_TOO_LARGE,
            StatusCode::URITooLong => Self::URI_TOO_LONG,
            StatusCode::UnsupportedMediaType => Self::UNSUPPORTED_MEDIA_TYPE,
            StatusCode::RangeNotSatisfiable => Self::RANGE_NOT_SATISFIABLE,
            StatusCode::ExpectationFailed => Self::EXPECTATION_FAILED,
            StatusCode::IAmATeapot => Self::IM_A_TEAPOT,
            StatusCode::MisdirectedRequest => Self::MISDIRECTED_REQUEST,
            StatusCode::UnprocessableEntity => Self::UNPROCESSABLE_ENTITY,
            StatusCode::Locked => Self::LOCKED,
            StatusCode::FailedDependency => Self::FAILED_DEPENDENCY,
            StatusCode::UpgradeRequired => Self::UPGRADE_REQUIRED,
            StatusCode::PreconditionRequired => Self::PRECONDITION_REQUIRED,
            StatusCode::TooManyRequests => Self::TOO_MANY_REQUESTS,
            StatusCode::RequestHeaderFieldsTooLarge => Self::REQUEST_HEADER_FIELDS_TOO_LARGE,
            StatusCode::UnavailableForLegalReasons => Self::UNAVAILABLE_FOR_LEGAL_REASONS,
            StatusCode::InternalServerError => Self::INTERNAL_SERVER_ERROR,
            StatusCode::NotImplemented => Self::NOT_IMPLEMENTED,
            StatusCode::BadGateway => Self::BAD_GATEWAY,
            StatusCode::ServiceUnavailable => Self::SERVICE_UNAVAILABLE,
            StatusCode::GatewayTimeout => Self::GATEWAY_TIMEOUT,
            StatusCode::HTTPVersionNotSupported => Self::HTTP_VERSION_NOT_SUPPORTED,
            StatusCode::VariantAlsoNegotiates => Self::VARIANT_ALSO_NEGOTIATES,
            StatusCode::InsufficientStorage => Self::INSUFFICIENT_STORAGE,
            StatusCode::LoopDetected => Self::LOOP_DETECTED,
            StatusCode::NotExtended => Self::NOT_EXTENDED,
            StatusCode::NetworkAuthenticationRequired => Self::NETWORK_AUTHENTICATION_REQUIRED,
            StatusCode::Custom(t) => Self::from_u16(t).unwrap_or(Self::INTERNAL_SERVER_ERROR),
        }
    }
}

macro_rules! automated_status_code {
    (
        $(
            $(#[$docs:meta])*
            ($konst:ident, $phrase:ident);
        )+
    ) => {
        $(
        pub const $konst: StatusCode = StatusCode::$phrase;
        )+
    }
}

automated_status_code! {
    /// 100 Continue
    /// [[RFC7231, Section 6.2.1](https://tools.ietf.org/html/rfc7231#section-6.2.1)]
    (CONTINUE,Continue);
    /// 101 SwitchingProtocols
    /// [[RFC7231, Section 6.2.2](https://tools.ietf.org/html/rfc7231#section-6.2.2)]
    (SWITCHING_PROTOCOLS,SwitchingProtocols);
    /// 102 Processing
    /// [[RFC2518](https://tools.ietf.org/html/rfc2518)]
    (PROCESSING,Processing);
    /// 200 OK
    /// [[RFC7231, Section 6.3.1](https://tools.ietf.org/html/rfc7231#section-6.3.1)]
    (OK,OK);
    /// 201 Created
    /// [[RFC7231, Section 6.3.2](https://tools.ietf.org/html/rfc7231#section-6.3.2)]
    (CREATED,Created);
    /// 202 Accepted
    /// [[RFC7231, Section 6.3.3](https://tools.ietf.org/html/rfc7231#section-6.3.3)]
    (ACCEPTED,Accepted);
    /// 203 Non-Authoritative Information
    /// [[RFC7231, Section 6.3.4](https://tools.ietf.org/html/rfc7231#section-6.3.4)]
    (NON_AUTHORITATIVE_INFORMATION,NonAuthoritativeInformation);
    /// 204 No Content
    /// [[RFC7231, Section 6.3.5](https://tools.ietf.org/html/rfc7231#section-6.3.5)]
    (NO_CONTENT,NoContent);
    /// 205 Reset Content
    /// [[RFC7231, Section 6.3.6](https://tools.ietf.org/html/rfc7231#section-6.3.6)]
    (RESET_CONTENT,ResetContent);
    /// 206 Partial Content
    /// [[RFC7233, Section 4.1](https://tools.ietf.org/html/rfc7233#section-4.1)]
    (PARTIAL_CONTENT,PartialContent);
    /// 207 Multi-Status
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (MULTI_STATUS,MultiStatus);
    /// 208 Already Reported
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    (ALREADY_REPORTED,AlreadyReported);
    /// 226 IM Used
    /// [[RFC3229](https://tools.ietf.org/html/rfc3229)]
    (IM_USED,IMUsed);
    /// 300 Multiple Choices
    /// [[RFC7231, Section 6.4.1](https://tools.ietf.org/html/rfc7231#section-6.4.1)]
    (MULTIPLE_CHOICES,MultipleChoices);
    /// 301 Moved Permanently
    /// [[RFC7231, Section 6.4.2](https://tools.ietf.org/html/rfc7231#section-6.4.2)]
    (MOVED_PERMANENTLY,MovedPermanently);
    /// 302 Found
    /// [[RFC7231, Section 6.4.3](https://tools.ietf.org/html/rfc7231#section-6.4.3)]
    (FOUND,Found);
    /// 303 See Other
    /// [[RFC7231, Section 6.4.4](https://tools.ietf.org/html/rfc7231#section-6.4.4)]
    (SEE_OTHER,SeeOther);
    /// 304 Not Modified
    /// [[RFC7232, Section 4.1](https://tools.ietf.org/html/rfc7232#section-4.1)]
    (NOT_MODIFIED,NotModified);
    /// 305 Use Proxy
    /// [[RFC7231, Section 6.4.5](https://tools.ietf.org/html/rfc7231#section-6.4.5)]
    (USE_PROXY,UseProxy);
    /// 307 Temporary Redirect
    /// [[RFC7231, Section 6.4.7](https://tools.ietf.org/html/rfc7231#section-6.4.7)]
    (TEMPORARY_REDIRECT,TemporaryRedirect);
    /// 308 Permanent Redirect
    /// [[RFC7238](https://tools.ietf.org/html/rfc7238)]
    (PERMANENT_REDIRECT,PermanentRedirect);
    /// 400 Bad Request
    /// [[RFC7231, Section 6.5.1](https://tools.ietf.org/html/rfc7231#section-6.5.1)]
    (BAD_REQUEST,BadRequest);
    /// 401 Unauthorized
    /// [[RFC7235, Section 3.1](https://tools.ietf.org/html/rfc7235#section-3.1)]
    (UNAUTHORIZED,Unauthorized);
    /// 402 Payment Required
    /// [[RFC7231, Section 6.5.2](https://tools.ietf.org/html/rfc7231#section-6.5.2)]
    (PAYMENT_REQUIRED,PaymentRequired);
    /// 403 Forbidden
    /// [[RFC7231, Section 6.5.3](https://tools.ietf.org/html/rfc7231#section-6.5.3)]
    (FORBIDDEN,Forbidden);
    /// 404 Not Found
    /// [[RFC7231, Section 6.5.4](https://tools.ietf.org/html/rfc7231#section-6.5.4)]
    (NOT_FOUND,NotFound);
    /// 405 Method Not Allowed
    /// [[RFC7231, Section 6.5.5](https://tools.ietf.org/html/rfc7231#section-6.5.5)]
    (METHOD_NOT_ALLOWED,MethodNotAllowed);
    /// 406 Not Acceptable
    /// [[RFC7231, Section 6.5.6](https://tools.ietf.org/html/rfc7231#section-6.5.6)]
    (NOT_ACCEPTABLE,NotAcceptable);
    /// 407 Proxy Authentication Required
    /// [[RFC7235, Section 3.2](https://tools.ietf.org/html/rfc7235#section-3.2)]
    (PROXY_AUTHENTICATION_REQUIRED,ProxyAuthenticationRequired);
    /// 408 Request Timeout
    /// [[RFC7231, Section 6.5.7](https://tools.ietf.org/html/rfc7231#section-6.5.7)]
    (REQUEST_TIMEOUT,RequestTimeout);
    /// 409 Conflict
    /// [[RFC7231, Section 6.5.8](https://tools.ietf.org/html/rfc7231#section-6.5.8)]
    (CONFLICT,Conflict);
    /// 410 Gone
    /// [[RFC7231, Section 6.5.9](https://tools.ietf.org/html/rfc7231#section-6.5.9)]
    (GONE,Gone);
    /// 411 Length Required
    /// [[RFC7231, Section 6.5.10](https://tools.ietf.org/html/rfc7231#section-6.5.10)]
    (LENGTH_REQUIRED,LengthRequired);
    /// 412 Precondition Failed
    /// [[RFC7232, Section 4.2](https://tools.ietf.org/html/rfc7232#section-4.2)]
    (PRECONDITION_FAILED,PreconditionFailed);
    /// 413 Payload Too Large
    /// [[RFC7231, Section 6.5.11](https://tools.ietf.org/html/rfc7231#section-6.5.11)]
    (PAYLOAD_TOO_LARGE,PayloadTooLarge);
    /// 414 URI Too Long
    /// [[RFC7231, Section 6.5.12](https://tools.ietf.org/html/rfc7231#section-6.5.12)]
    (URI_TOO_LONG,URITooLong);
    /// 415 Unsupported Media Type
    /// [[RFC7231, Section 6.5.13](https://tools.ietf.org/html/rfc7231#section-6.5.13)]
    (UNSUPPORTED_MEDIA_TYPE,UnsupportedMediaType);
    /// 416 Range Not Satisfiable
    /// [[RFC7233, Section 4.4](https://tools.ietf.org/html/rfc7233#section-4.4)]
    (RANGE_NOT_SATISFIABLE,RangeNotSatisfiable);
    /// 417 Expectation Failed
    /// [[RFC7231, Section 6.5.14](https://tools.ietf.org/html/rfc7231#section-6.5.14)]
    (EXPECTATION_FAILED,ExpectationFailed);
    /// 418 I'm a teapot
    /// [curiously not registered by IANA but [RFC2324](https://tools.ietf.org/html/rfc2324)]
    (IM_A_TEAPOT,IAmATeapot);
    /// 421 Misdirected Request
    /// [RFC7540, Section 9.1.2](http://tools.ietf.org/html/rfc7540#section-9.1.2)
    (MISDIRECTED_REQUEST,MisdirectedRequest);
    /// 422 Unprocessable Entity
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (UNPROCESSABLE_ENTITY,UnprocessableEntity);
    /// 423 Locked
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (LOCKED,Locked);
    /// 424 Failed Dependency
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (FAILED_DEPENDENCY,FailedDependency);
    /// 426 Upgrade Required
    /// [[RFC7231, Section 6.5.15](https://tools.ietf.org/html/rfc7231#section-6.5.15)]
    (UPGRADE_REQUIRED,UpgradeRequired);
    /// 428 Precondition Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (PRECONDITION_REQUIRED,PreconditionRequired);
    /// 429 Too Many Requests
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (TOO_MANY_REQUESTS,TooManyRequests);
    /// 431 Request Header Fields Too Large
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (REQUEST_HEADER_FIELDS_TOO_LARGE,RequestHeaderFieldsTooLarge);
    /// 451 Unavailable For Legal Reasons
    /// [[RFC7725](http://tools.ietf.org/html/rfc7725)]
    (UNAVAILABLE_FOR_LEGAL_REASONS,UnavailableForLegalReasons);
    /// 500 Internal Server Error
    /// [[RFC7231, Section 6.6.1](https://tools.ietf.org/html/rfc7231#section-6.6.1)]
    (INTERNAL_SERVER_ERROR,InternalServerError);
    /// 501 Not Implemented
    /// [[RFC7231, Section 6.6.2](https://tools.ietf.org/html/rfc7231#section-6.6.2)]
    (NOT_IMPLEMENTED,NotImplemented);
    /// 502 Bad Gateway
    /// [[RFC7231, Section 6.6.3](https://tools.ietf.org/html/rfc7231#section-6.6.3)]
    (BAD_GATEWAY,BadGateway);
    /// 503 Service Unavailable
    /// [[RFC7231, Section 6.6.4](https://tools.ietf.org/html/rfc7231#section-6.6.4)]
    (SERVICE_UNAVAILABLE,ServiceUnavailable);
    /// 504 Gateway Timeout
    /// [[RFC7231, Section 6.6.5](https://tools.ietf.org/html/rfc7231#section-6.6.5)]
    (GATEWAY_TIMEOUT,GatewayTimeout);
    /// 505 HTTP Version Not Supported
    /// [[RFC7231, Section 6.6.6](https://tools.ietf.org/html/rfc7231#section-6.6.6)]
    (HTTP_VERSION_NOT_SUPPORTED,HTTPVersionNotSupported);
    /// 506 Variant Also Negotiates
    /// [[RFC2295](https://tools.ietf.org/html/rfc2295)]
    (VARIANT_ALSO_NEGOTIATES,VariantAlsoNegotiates);
    /// 507 Insufficient Storage
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (INSUFFICIENT_STORAGE,InsufficientStorage);
    /// 508 Loop Detected
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    (LOOP_DETECTED,LoopDetected);
    /// 510 Not Extended
    /// [[RFC2774](https://tools.ietf.org/html/rfc2774)]
    (NOT_EXTENDED,NotExtended);
    /// 511 Network Authentication Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (NETWORK_AUTHENTICATION_REQUIRED,NetworkAuthenticationRequired);
}

impl serde::Serialize for StatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(u16::from(self))
    }
}

pub struct U16New(u16);

impl<'de> serde::Deserialize<'de> for StatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<'de> {
            marker: PhantomData<StatusCode>,
            lifetime: PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for Visitor<'de> {
            type Value = StatusCode;
            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                Formatter::write_str(formatter, "StatusCode within u16 spec")
            }
            #[inline]
            fn visit_newtype_struct<E>(self, e: E) -> Result<Self::Value, E::Error>
            where
                E: serde::Deserializer<'de>,
            {
                let deserialize_field: u16 = match <u16 as serde::Deserialize>::deserialize(e) {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(err);
                    }
                };
                Ok(StatusCode::from(deserialize_field))
            }
            #[inline]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let deserialize_field =
                    match match serde::de::SeqAccess::next_element::<u16>(&mut seq) {
                        Ok(val) => val,
                        Err(err) => {
                            return Err(err);
                        }
                    } {
                        Some(value) => value,
                        None => {
                            return Err(serde::de::Error::invalid_length(
                                0usize,
                                &"StatusCode id within u16 spec",
                            ));
                        }
                    };
                Ok(StatusCode::from(deserialize_field))
            }
        }
        serde::Deserializer::deserialize_newtype_struct(
            deserializer,
            "StatusCode",
            Visitor {
                marker: PhantomData::<StatusCode>,
                lifetime: PhantomData,
            },
        )
    }
}
