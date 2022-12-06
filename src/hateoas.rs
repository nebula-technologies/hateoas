use crate::frameworks::actix::error::ActixError;
use crate::resource_trait::HateoasResource;
use crate::serde::Serialize;
use crate::{Content, Metadata, Status};
use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub struct Hateoas<T: HateoasResource> {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    metadata: Option<Metadata>,
    spec: Option<Content<T>>,
    status: Option<Status>,
}

impl<T: HateoasResource> Hateoas<T> {
    /// ## New Hateoas.
    /// this will create a new instance of Hateoas that will make it easier to crate API replyes for services.
    ///
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let new_hateoas_response: Hateoas<String> = Hateoas::new(None, None, None);
    ///
    /// assert_eq!(new_hateoas_response, Hateoas::default() as Hateoas<String>);
    /// assert_eq!(None, new_hateoas_response.spec());
    /// assert_eq!(None, new_hateoas_response.status());
    /// assert_eq!(None, new_hateoas_response.metadata());
    /// assert_eq!(Some(&"String".to_string()), new_hateoas_response.kind());
    /// assert_eq!(Some(&"hateoas.io/0.0.1".to_string()), new_hateoas_response.api_version());
    /// ```
    pub fn new(
        spec: Option<Content<T>>,
        metadata: Option<Metadata>,
        status: Option<Status>,
    ) -> Self {
        Self {
            api_version: format!("{}/{}", T::GROUP, T::VERSION),
            kind: format!("{}", T::KIND),
            metadata,
            spec,
            status,
        }
    }

    /// ## Getting the kind of the resource
    /// This will return the kind of the resource requested.
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let new_hateoas_response: Hateoas<String> = Hateoas::new(None, None, None);
    ///
    /// assert_eq!(Some(&"String".to_string()), new_hateoas_response.kind());
    /// ```
    pub fn kind(&self) -> &String {
        &self.kind
    }

    /// ## Getting the api version of the resource
    /// This will get the API version of the resource, this will help as API expands and there are
    /// multiple versions on the system.
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let new_hateoas_response: Hateoas<String> = Hateoas::new(None, None, None);
    ///
    /// assert_eq!(Some(&"hateoas.io/0.0.1".to_string()), new_hateoas_response.api_version());
    /// ```
    pub fn api_version(&self) -> &String {
        &self.api_version
    }

    /// ## Getter for the metadata property
    /// This will get the metadata, if there is no metadata in the system, metadata will no be
    /// initialized, in comparison to the setter `metadata_mut` that will initialize the metadata
    /// property with the metadata object.
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let hateoas: Hateoas<()> = Hateoas::default();
    /// let metadata = hateoas.metadata();
    ///
    /// assert_eq!(None, metadata)
    /// ```
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }
    /// Getting the metadata from the response.
    /// By default metadata is not initialized and will be initialized upon usage.
    /// ```
    /// use hateoas::{Hateoas, Metadata};
    ///
    /// let mut response: Hateoas<()> = Hateoas::default();
    /// let mut metadata = Metadata::default();
    ///
    /// assert_eq!(Some(&mut metadata), response.metadata_mut());
    /// ```
    pub fn metadata_mut(&mut self) -> &mut Metadata {
        self.metadata.get_or_insert(Metadata::default())
    }

    /// ## Getter for the status property
    /// This will get the status, if there is no status in the system, status will no be
    /// initialized, in comparison to the setter `status_mut` that will initialize the status
    /// property with the status object.
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let hateoas: Hateoas<()> = Hateoas::default();
    /// let status = hateoas.status();
    ///
    /// assert_eq!(None, status)
    /// ```
    pub fn status(&self) -> Option<&Status> {
        self.status.as_ref()
    }
    /// Get The status information from the response,
    /// If this is not initialized it will be initialized and returned.
    /// ```
    /// use hateoas::{Hateoas, Status};
    ///
    /// let mut response: Hateoas<()> = Hateoas::default();
    ///
    /// let mut status = response.status_mut();
    /// assert_eq!(Some(&mut Status::default()), status)
    /// ```
    pub fn status_mut(&mut self) -> &mut Status {
        self.status.get_or_insert(Status::default())
    }

    /// ## Getter for the spec property
    /// This will get the spec, if there is no spec in the system, spec will no be
    /// initialized, in comparison to the setter `spec_mut` that will initialize the spec
    /// property with the spec object.
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let hateoas: Hateoas<()> = Hateoas::default();
    /// let spec = hateoas.spec();
    ///
    /// assert_eq!(None, spec)
    /// ```
    pub fn spec(&self) -> Option<&Content<T>> {
        self.spec.as_ref()
    }

    /// Get the spec information form the Response payload
    ///
    /// ```
    /// use hateoas::{Content, Hateoas};
    ///
    /// let mut response: Hateoas<String> = Hateoas::default();
    ///
    /// // Here spec will be None at initialization time.
    /// // at [Response.spec_mut()] Spec will be initialized and returned.
    ///
    /// let mut spec = response.spec_mut();
    /// assert_eq!(Some(&mut Content::default()), spec)
    /// ```
    pub fn spec_mut(&mut self) -> &mut Content<T> {
        self.spec.get_or_insert(Content::default())
    }
}

impl From<ActixError> for Hateoas<()> {
    fn from(e: ActixError) -> Self {
        match e {
            ActixError::OverflowKnownLength { .. } => Hateoas::PAYLOAD_TOO_LARGE(
                None,
                Some("Content not matching expected length".to_string()),
            ),
            ActixError::Overflow { .. } => {
                Hateoas::PAYLOAD_TOO_LARGE(None, Some("Payload too large".to_string()))
            }
            ActixError::ContentType => {
                Hateoas::UNPROCESSABLE_ENTITY(None, Some("Unknown content type".to_string()))
            }
            ActixError::Deserialize(_) => {
                Hateoas::UNPROCESSABLE_ENTITY(None, Some("Unknown format".to_string()))
            }
            ActixError::Serialize(_) => {
                Hateoas::UNPROCESSABLE_ENTITY(None, Some("Unknown format".to_string()))
            }
            ActixError::Payload(_) => Hateoas::INTERNAL_SERVER_ERROR(None, None),
            ActixError::PayloadError(_, _) => Hateoas::INTERNAL_SERVER_ERROR(None, None),
            ActixError::NoPayloadSizeDefinitionInHeader => {
                Hateoas::INTERNAL_SERVER_ERROR(None, None)
            }
            ActixError::FailedToMapHeaderToStr(_) => Hateoas::INTERNAL_SERVER_ERROR(None, None),
            ActixError::SerializationDeserializationError(_) => {
                Hateoas::UNPROCESSABLE_ENTITY(None, Some("Unknown format".to_string()))
            }
            ActixError::Infallible => Hateoas::INTERNAL_SERVER_ERROR(None, None),
            ActixError::FailedToParseToInt(_) => Hateoas::INTERNAL_SERVER_ERROR(None, None),
            ActixError::FailedToGetContentTypeFromHeader => {
                Hateoas::INTERNAL_SERVER_ERROR(None, None)
            }
        }
    }
}

impl<T: Serialize + HateoasResource> From<T> for Hateoas<T> {
    fn from(t: T) -> Self {
        Hateoas::OK(Some(t), None)
    }
}

impl<T: HateoasResource> Default for Hateoas<T> {
    fn default() -> Self {
        Self {
            api_version: format!("{}/{}", T::GROUP, T::VERSION),
            kind: format!("{}", T::KIND),
            metadata: None,
            spec: None,
            status: None,
        }
    }
}

macro_rules! automated_code_hateoas {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl<T: HateoasResource> Hateoas<T> {
        $(
            $(#[$docs])*
            #[doc = " ```\n" ]
            #[doc = " use hateoas::{Hateoas,Content,Status};\n"]
            #[doc = " \n" ]
            #[doc = concat!(" let hateoas: Hateoas<String> = Hateoas::", stringify!($konst), "(Some(", stringify!($phrase), ".to_string()));\n") ]
            #[doc = " \n" ]
            #[doc = concat!(" assert_eq!(hateoas, Hateoas::new(Some(Content::new(", stringify!($phrase), ".to_string())), None, Some(Status::", stringify!($konst), "())));\n") ]
            #[doc = " ``` "]
            #[allow(non_snake_case)]
            pub fn $konst(data: Option<T>, msg: Option<String>) -> Self {
                Self::new(data.map(|t| Content::new(t)), None, Some(Status::$konst(msg)))
            }

        )+
        }
    }
}

automated_code_hateoas! {
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

#[cfg(test)]
mod test {
    use crate::{Content, Hateoas, HateoasResource, RelLinkCollection};

    #[derive(Serialize, Deserialize)]
    pub struct RubberBullet {
        pub name: String,
        pub title: String,
        pub chapter: String,
    }

    impl Default for RubberBullet {
        fn default() -> Self {
            RubberBullet {
                name: "Rubber Bullet".to_string(),
                title: "The Bullet".to_string(),
                chapter: "A Rubber Bullet Hurts".to_string(),
            }
        }
    }

    impl HateoasResource for RubberBullet {
        const KIND: &'static str = "";
        const VERSION: &'static str = "";
        const GROUP: &'static str = "";
        const URL_PATH_SEGMENT: &'static str = "";
    }

    const RUBBER_BULLET_SER: &str = r#"{
      "apiVersion": "/",
      "kind": "",
      "metadata": null,
      "spec": {
        "content": {
          "name": "Rubber Bullet",
          "title": "The Bullet",
          "chapter": "A Rubber Bullet Hurts"
        },
        "rel": null
      },
      "status": {
        "message": "OK",
        "code": null,
        "http_status_code": 200,
        "session": null
      }
    }"#;
    #[test]
    pub fn confirm_some_code() {}

    #[test]
    pub fn serialize_test() {
        let rubber_bullet = RubberBullet {
            name: "Rubber Bullet".to_string(),
            title: "The Bullet".to_string(),
            chapter: "A Rubber Bullet Hurts".to_string(),
        };

        let response = Hateoas::OK(Some(rubber_bullet), None);

        let response_ser: serde_json::Value = serde_json::to_value(&response).unwrap();

        println!("{:#?}", response_ser);
    }
    #[test]
    pub fn deserialize_test() {
        let response_ser: serde_json::Value = serde_json::from_str(RUBBER_BULLET_SER).unwrap();

        println!("{:#?}", response_ser);
    }

    // #[test]
    // fn default_response_test() {
    //     let response: Response<String> = Response {
    //         content: None,
    //         metadata: Default::default(),
    //     };
    //     let response_ser = response.encode("yaml");
    //
    //     // println!("{}", response_ser);
    //     // assert_eq!()
    // }

    #[test]
    fn test_content_rel() {
        let mut content: Content<()> = Content::default();
        let rel = content.rel();
        assert_eq!(&mut RelLinkCollection::default(), rel);
    }

    #[test]
    fn test_get_spec_on_none() {
        let mut response: Hateoas<String> = Hateoas::default();

        // Here spec will be None at initialization time.
        // at [Response.spec()] Spec will be initialized and returned.

        let mut spec = response.spec_mut();
        assert_eq!(Some(&mut Content::default()), spec)
    }
}
