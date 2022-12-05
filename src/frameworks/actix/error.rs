use actix_web::error::PayloadError;
use actix_web::http::header::ToStrError;

#[derive(Debug, Display)]
#[non_exhaustive]
pub enum ActixError {
    /// Payload size is bigger than allowed & content length header set. (default: 2MB)
    #[display(
        fmt = "Ricksponse payload ({} bytes) is larger than allowed (limit: {} bytes).",
        length,
        limit
    )]
    OverflowKnownLength { length: usize, limit: usize },

    /// Payload size is bigger than allowed but no content length header set. (default: 2MB)
    #[display(fmt = "payload has exceeded limit ({} bytes).", limit)]
    Overflow { limit: usize },

    /// Content type error
    #[display(fmt = "Content type error")]
    ContentType,

    /// Deserialize error
    #[display(fmt = "Deserialize error: {:?}", _0)]
    Deserialize(simple_serde::Error),

    /// Serialize error
    #[display(fmt = "Serialize error: {:?}", _0)]
    Serialize(simple_serde::Error),

    /// Payload error
    #[display(fmt = "Error that occur during reading payload: {}", _0)]
    Payload(PayloadError),

    /// Payload error
    #[display(
        fmt = "Failed to deserialize payload under future stream assembly. Request path {}, Error {}",
        _0,
        _1
    )]
    PayloadError(String, Box<ActixError>),

    #[display(fmt = "Content-Length is missing in the headers")]
    NoPayloadSizeDefinitionInHeader,

    #[display(fmt = "Header cannot be mapped to String: {:?}", _0)]
    FailedToMapHeaderToStr(ToStrError),

    #[display(fmt = "Serializer/Deserializer error: {}", _0)]
    SerializationDeserializationError(simple_serde::Error),

    #[display(fmt = "Infallible - this should not happen")]
    Infallible,

    #[display(fmt = "Failed to pars content to Integer: {}", _0)]
    FailedToParseToInt(std::num::ParseIntError),

    #[display(fmt = "Failed to get Content-Type from header")]
    FailedToGetContentTypeFromHeader,
}

impl From<ToStrError> for ActixError {
    fn from(e: ToStrError) -> Self {
        Self::FailedToMapHeaderToStr(e)
    }
}

impl From<simple_serde::Error> for ActixError {
    fn from(e: simple_serde::Error) -> Self {
        Self::SerializationDeserializationError(e)
    }
}

impl From<std::convert::Infallible> for ActixError {
    fn from(_: std::convert::Infallible) -> Self {
        Self::Infallible
    }
}

impl From<std::num::ParseIntError> for ActixError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::FailedToParseToInt(e)
    }
}
