#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Status {
    pub(crate) message: Option<String>,
    pub(crate) code: Option<u32>,
    pub(crate) http_status_code: Option<u16>,
    pub(crate) session: Option<uuid::Uuid>,
}

impl Status {
    pub fn new(
        message: Option<&str>,
        code: Option<u32>,
        http_status_code: Option<u16>,
        session: Option<uuid::Uuid>,
    ) -> Self {
        Status {
            message: message.map(|t| t.to_string()),
            code,
            http_status_code,
            session,
        }
    }

    pub const fn const_new(
        message: Option<String>,
        code: Option<u32>,
        http_status_code: Option<u16>,
        session: Option<uuid::Uuid>,
    ) -> Self {
        Status {
            message,
            code,
            http_status_code,
            session,
        }
    }

    /// ## Getting Message
    /// This is for getting the message field from the status object.
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::new(Some("hello world"), None, None, None);
    ///
    /// assert_eq!(status.message(), &Some("hello world".to_string()));
    /// ```
    pub fn message(&self) -> &Option<String> {
        &self.message
    }

    /// ## Getting Mutable Message
    /// This is for getting the message field from the status object.
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::new(Some("hello world"), None, None, None);
    ///
    /// let mut mut_message = status.message_mut();
    /// *mut_message = Some("Hello Space".to_string());
    ///
    /// assert_eq!(status.message(), &Some("Hello Space".to_string()));
    /// ```
    pub fn message_mut(&mut self) -> &mut Option<String> {
        &mut self.message
    }

    /// ## Getting code
    /// Getting the internal status code from the stauts object
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::new(None, Some(200), None, None);
    ///
    /// assert_eq!(status.code(), &Some(200));
    /// ```
    pub fn code(&self) -> &Option<u32> {
        &self.code
    }

    /// ## Getting Mutable Code
    /// Getting the internal status code from the stauts object as a mutable reference
    /// allowing for modifications to the internal status code.
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::new(None, Some(200), None, None);
    ///
    /// let mut status_code = status.code_mut();
    /// *status_code = Some(100);
    ///
    /// assert_eq!(status.code(), &Some(100));
    /// ```
    pub fn code_mut(&mut self) -> &mut Option<u32> {
        &mut self.code
    }

    /// ## Getter for the HTTP status code
    /// This is for getting the http_status_code.
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::new(None, None, Some(200), None);
    ///
    /// assert_eq!(status.http_status_code(), &Some(200));
    /// ```
    pub fn http_status_code(&self) -> &Option<u16> {
        &self.http_status_code
    }

    /// ## Getter for mutable HTTP status code
    /// This is for getting the http_status_code.
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let mut status = Status::new(None, None, Some(200), None);
    ///
    /// let mut http_code = status.http_status_code_mut();
    /// *http_code = Some(100);
    ///
    /// assert_eq!(status.http_status_code(), &Some(100));
    /// ```
    pub fn http_status_code_mut(&mut self) -> &mut Option<u16> {
        &mut self.http_status_code
    }

    /// ## Getter for the Session
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let uuid = uuid::Uuid::new_v4();
    /// let mut status = Status::new(None, None, None, Some(uuid));
    ///
    /// assert_eq!(status.session(), &Some(uuid));
    /// ```
    pub fn session(&self) -> &Option<uuid::Uuid> {
        &self.session
    }

    /// ## Getter for mutable Session id
    ///
    /// ```
    /// use hateoas::Status;
    ///
    /// let uuid = uuid::Uuid::new_v4();
    /// let uuid_2 = uuid::Uuid::new_v4();
    /// let mut status = Status::default();
    ///
    /// let mut mut_session = status.session_mut();
    /// *mut_session = Some(uuid_2);
    ///
    /// assert_eq!(status.session(), &Some(uuid_2));
    /// ```
    pub fn session_mut(&mut self) -> &mut Option<uuid::Uuid> {
        &mut self.session
    }

    pub fn get(
        &self,
    ) -> (
        &Option<String>,
        &Option<u32>,
        &Option<u16>,
        &Option<uuid::Uuid>,
    ) {
        (
            &self.message,
            &self.code,
            &self.http_status_code,
            &self.session,
        )
    }

    pub fn get_mut(
        &mut self,
    ) -> (
        &mut Option<String>,
        &mut Option<u32>,
        &mut Option<u16>,
        &mut Option<uuid::Uuid>,
    ) {
        (
            &mut self.message,
            &mut self.code,
            &mut self.http_status_code,
            &mut self.session,
        )
    }
}

macro_rules! automated_status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl Status {
        $(
            $(#[$docs])*
            #[doc = " ```\n" ]
            #[doc = " use hateoas::Status;\n"]
            #[doc = " \n" ]
            #[doc = concat!(" let status = Status::", stringify!($konst), "(None);\n") ]
            #[doc = " \n" ]
            #[doc = concat!(" assert_eq!(status, Status::const_new(Some(", stringify!($phrase), ".to_string()), None, Some(", stringify!($num), "), None));\n") ]
            #[doc = " ``` "]
            #[allow(non_snake_case)]
             pub fn $konst(msg: Option<String>) -> Self {
                Self::const_new(
                    msg.or_else(|| Some($phrase.to_string())),
                    None,
                    Some($num),
                    None
                )
            }
        )+
        }
    }
}

automated_status_codes! {
    /// 100 Continue
    /// [[RFC7231, Section 6.2.1](https://tools.ietf.org/html/rfc7231#section-6.2.1)]
    (100, CONTINUE, "Continue");
    /// 101 Switching Protocols
    /// [[RFC7231, Section 6.2.2](https://tools.ietf.org/html/rfc7231#section-6.2.2)]
    (101, SWITCHING_PROTOCOLS, "Switching Protocols");
    /// 102 Processing
    /// [[RFC2518](https://tools.ietf.org/html/rfc2518)]
    (102, PROCESSING, "Processing");

    /// 200 OK
    /// [[RFC7231, Section 6.3.1](https://tools.ietf.org/html/rfc7231#section-6.3.1)]
    (200, OK, "OK");
    /// 201 Created
    /// [[RFC7231, Section 6.3.2](https://tools.ietf.org/html/rfc7231#section-6.3.2)]
    (201, CREATED, "Created");
    /// 202 Accepted
    /// [[RFC7231, Section 6.3.3](https://tools.ietf.org/html/rfc7231#section-6.3.3)]
    (202, ACCEPTED, "Accepted");
    /// 203 Non-Authoritative Information
    /// [[RFC7231, Section 6.3.4](https://tools.ietf.org/html/rfc7231#section-6.3.4)]
    (203, NON_AUTHORITATIVE_INFORMATION, "Non Authoritative Information");
    /// 204 No Content
    /// [[RFC7231, Section 6.3.5](https://tools.ietf.org/html/rfc7231#section-6.3.5)]
    (204, NO_CONTENT, "No Content");
    /// 205 Reset Content
    /// [[RFC7231, Section 6.3.6](https://tools.ietf.org/html/rfc7231#section-6.3.6)]
    (205, RESET_CONTENT, "Reset Content");
    /// 206 Partial Content
    /// [[RFC7233, Section 4.1](https://tools.ietf.org/html/rfc7233#section-4.1)]
    (206, PARTIAL_CONTENT, "Partial Content");
    /// 207 Multi-Status
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (207, MULTI_STATUS, "Multi-Status");
    /// 208 Already Reported
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    (208, ALREADY_REPORTED, "Already Reported");

    /// 226 IM Used
    /// [[RFC3229](https://tools.ietf.org/html/rfc3229)]
    (226, IM_USED, "IM Used");

    /// 300 Multiple Choices
    /// [[RFC7231, Section 6.4.1](https://tools.ietf.org/html/rfc7231#section-6.4.1)]
    (300, MULTIPLE_CHOICES, "Multiple Choices");
    /// 301 Moved Permanently
    /// [[RFC7231, Section 6.4.2](https://tools.ietf.org/html/rfc7231#section-6.4.2)]
    (301, MOVED_PERMANENTLY, "Moved Permanently");
    /// 302 Found
    /// [[RFC7231, Section 6.4.3](https://tools.ietf.org/html/rfc7231#section-6.4.3)]
    (302, FOUND, "Found");
    /// 303 See Other
    /// [[RFC7231, Section 6.4.4](https://tools.ietf.org/html/rfc7231#section-6.4.4)]
    (303, SEE_OTHER, "See Other");
    /// 304 Not Modified
    /// [[RFC7232, Section 4.1](https://tools.ietf.org/html/rfc7232#section-4.1)]
    (304, NOT_MODIFIED, "Not Modified");
    /// 305 Use Proxy
    /// [[RFC7231, Section 6.4.5](https://tools.ietf.org/html/rfc7231#section-6.4.5)]
    (305, USE_PROXY, "Use Proxy");
    /// 307 Temporary Redirect
    /// [[RFC7231, Section 6.4.7](https://tools.ietf.org/html/rfc7231#section-6.4.7)]
    (307, TEMPORARY_REDIRECT, "Temporary Redirect");
    /// 308 Permanent Redirect
    /// [[RFC7238](https://tools.ietf.org/html/rfc7238)]
    (308, PERMANENT_REDIRECT, "Permanent Redirect");

    /// 400 Bad Request
    /// [[RFC7231, Section 6.5.1](https://tools.ietf.org/html/rfc7231#section-6.5.1)]
    (400, BAD_REQUEST, "Bad Request");
    /// 401 Unauthorized
    /// [[RFC7235, Section 3.1](https://tools.ietf.org/html/rfc7235#section-3.1)]
    (401, UNAUTHORIZED, "Unauthorized");
    /// 402 Payment Required
    /// [[RFC7231, Section 6.5.2](https://tools.ietf.org/html/rfc7231#section-6.5.2)]
    (402, PAYMENT_REQUIRED, "Payment Required");
    /// 403 Forbidden
    /// [[RFC7231, Section 6.5.3](https://tools.ietf.org/html/rfc7231#section-6.5.3)]
    (403, FORBIDDEN, "Forbidden");
    /// 404 Not Found
    /// [[RFC7231, Section 6.5.4](https://tools.ietf.org/html/rfc7231#section-6.5.4)]
    (404, NOT_FOUND, "Not Found");
    /// 405 Method Not Allowed
    /// [[RFC7231, Section 6.5.5](https://tools.ietf.org/html/rfc7231#section-6.5.5)]
    (405, METHOD_NOT_ALLOWED, "Method Not Allowed");
    /// 406 Not Acceptable
    /// [[RFC7231, Section 6.5.6](https://tools.ietf.org/html/rfc7231#section-6.5.6)]
    (406, NOT_ACCEPTABLE, "Not Acceptable");
    /// 407 Proxy Authentication Required
    /// [[RFC7235, Section 3.2](https://tools.ietf.org/html/rfc7235#section-3.2)]
    (407, PROXY_AUTHENTICATION_REQUIRED, "Proxy Authentication Required");
    /// 408 Request Timeout
    /// [[RFC7231, Section 6.5.7](https://tools.ietf.org/html/rfc7231#section-6.5.7)]
    (408, REQUEST_TIMEOUT, "Request Timeout");
    /// 409 Conflict
    /// [[RFC7231, Section 6.5.8](https://tools.ietf.org/html/rfc7231#section-6.5.8)]
    (409, CONFLICT, "Conflict");
    /// 410 Gone
    /// [[RFC7231, Section 6.5.9](https://tools.ietf.org/html/rfc7231#section-6.5.9)]
    (410, GONE, "Gone");
    /// 411 Length Required
    /// [[RFC7231, Section 6.5.10](https://tools.ietf.org/html/rfc7231#section-6.5.10)]
    (411, LENGTH_REQUIRED, "Length Required");
    /// 412 Precondition Failed
    /// [[RFC7232, Section 4.2](https://tools.ietf.org/html/rfc7232#section-4.2)]
    (412, PRECONDITION_FAILED, "Precondition Failed");
    /// 413 Payload Too Large
    /// [[RFC7231, Section 6.5.11](https://tools.ietf.org/html/rfc7231#section-6.5.11)]
    (413, PAYLOAD_TOO_LARGE, "Payload Too Large");
    /// 414 URI Too Long
    /// [[RFC7231, Section 6.5.12](https://tools.ietf.org/html/rfc7231#section-6.5.12)]
    (414, URI_TOO_LONG, "URI Too Long");
    /// 415 Unsupported Media Type
    /// [[RFC7231, Section 6.5.13](https://tools.ietf.org/html/rfc7231#section-6.5.13)]
    (415, UNSUPPORTED_MEDIA_TYPE, "Unsupported Media Type");
    /// 416 Range Not Satisfiable
    /// [[RFC7233, Section 4.4](https://tools.ietf.org/html/rfc7233#section-4.4)]
    (416, RANGE_NOT_SATISFIABLE, "Range Not Satisfiable");
    /// 417 Expectation Failed
    /// [[RFC7231, Section 6.5.14](https://tools.ietf.org/html/rfc7231#section-6.5.14)]
    (417, EXPECTATION_FAILED, "Expectation Failed");
    /// 418 I'm a teapot
    /// [curiously not registered by IANA but [RFC2324](https://tools.ietf.org/html/rfc2324)]
    (418, IM_A_TEAPOT, "I'm a teapot");

    /// 421 Misdirected Request
    /// [RFC7540, Section 9.1.2](http://tools.ietf.org/html/rfc7540#section-9.1.2)
    (421, MISDIRECTED_REQUEST, "Misdirected Request");
    /// 422 Unprocessable Entity
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (422, UNPROCESSABLE_ENTITY, "Unprocessable Entity");
    /// 423 Locked
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (423, LOCKED, "Locked");
    /// 424 Failed Dependency
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (424, FAILED_DEPENDENCY, "Failed Dependency");

    /// 426 Upgrade Required
    /// [[RFC7231, Section 6.5.15](https://tools.ietf.org/html/rfc7231#section-6.5.15)]
    (426, UPGRADE_REQUIRED, "Upgrade Required");

    /// 428 Precondition Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (428, PRECONDITION_REQUIRED, "Precondition Required");
    /// 429 Too Many Requests
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (429, TOO_MANY_REQUESTS, "Too Many Requests");

    /// 431 Request Header Fields Too Large
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (431, REQUEST_HEADER_FIELDS_TOO_LARGE, "Request Header Fields Too Large");

    /// 451 Unavailable For Legal Reasons
    /// [[RFC7725](http://tools.ietf.org/html/rfc7725)]
    (451, UNAVAILABLE_FOR_LEGAL_REASONS, "Unavailable For Legal Reasons");

    /// 500 Internal Server Error
    /// [[RFC7231, Section 6.6.1](https://tools.ietf.org/html/rfc7231#section-6.6.1)]
    (500, INTERNAL_SERVER_ERROR, "Internal Server Error");
    /// 501 Not Implemented
    /// [[RFC7231, Section 6.6.2](https://tools.ietf.org/html/rfc7231#section-6.6.2)]
    (501, NOT_IMPLEMENTED, "Not Implemented");
    /// 502 Bad Gateway
    /// [[RFC7231, Section 6.6.3](https://tools.ietf.org/html/rfc7231#section-6.6.3)]
    (502, BAD_GATEWAY, "Bad Gateway");
    /// 503 Service Unavailable
    /// [[RFC7231, Section 6.6.4](https://tools.ietf.org/html/rfc7231#section-6.6.4)]
    (503, SERVICE_UNAVAILABLE, "Service Unavailable");
    /// 504 Gateway Timeout
    /// [[RFC7231, Section 6.6.5](https://tools.ietf.org/html/rfc7231#section-6.6.5)]
    (504, GATEWAY_TIMEOUT, "Gateway Timeout");
    /// 505 HTTP Version Not Supported
    /// [[RFC7231, Section 6.6.6](https://tools.ietf.org/html/rfc7231#section-6.6.6)]
    (505, HTTP_VERSION_NOT_SUPPORTED, "HTTP Version Not Supported");
    /// 506 Variant Also Negotiates
    /// [[RFC2295](https://tools.ietf.org/html/rfc2295)]
    (506, VARIANT_ALSO_NEGOTIATES, "Variant Also Negotiates");
    /// 507 Insufficient Storage
    /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
    (507, INSUFFICIENT_STORAGE, "Insufficient Storage");
    /// 508 Loop Detected
    /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
    (508, LOOP_DETECTED, "Loop Detected");

    /// 510 Not Extended
    /// [[RFC2774](https://tools.ietf.org/html/rfc2774)]
    (510, NOT_EXTENDED, "Not Extended");
    /// 511 Network Authentication Required
    /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
    (511, NETWORK_AUTHENTICATION_REQUIRED, "Network Authentication Required");
}
