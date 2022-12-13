use crate::{Hateoas, HateoasResource, HeaderMap};
use axum::body::HttpBody;
use axum::extract::FromRequest;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum_core::extract::rejection::BytesRejection;
use axum_core::extract::FromRequestParts;
use axum_core::BoxError;
use bytes::Bytes;
use railsgun::rail_tap::TapClone;
use railsgun::TapRef;
use serde::de::DeserializeOwned;
use serde::Serialize;
use simple_serde::{Decoded, Error, SimpleDecoder};
use std::future::Future;
use std::pin::Pin;
use tracing::{event, span, Level};

impl<T: Serialize + HateoasResource> IntoResponse for Hateoas<T> {
    fn into_response(self) -> Response {
        ().into_response()
    }
}

// impl<T: DeserializeOwned + HateoasResource, S, B> FromRequest<S, B> for Hateoas<T> {
//     type Rejection = ();
// }

impl<S, B, T: DeserializeOwned + HateoasResource> FromRequest<S, B> for Hateoas<T>
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
    T: FromRequestParts<S>,
{
    type Rejection = Hateoas<()>;

    fn from_request<'a, 'b>(
        req: Request<B>,
        state: &'a S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'b>>
    where
        'a: 'b,
        Self: 'b,
    {
        Box::pin(async move {
            let headers = HeaderMap::from(req.headers());
            let content_type = headers
                .get_first("Content-Type")
                .and_then(|t| String::from_utf8(t.to_vec()).ok())
                .unwrap();

            let bytes = Bytes::from_request(req, state)
                .await
                .map_err(Hateoas::from)?
                .to_vec();
            bytes
                .decode(&content_type)
                .map(|t: Decoded<Hateoas<T>>| t.into())
                .or_else(|t| {
                    event!(Level::WARN, "Failed to decode the payload, will try again.");
                    bytes
                        .decode(&content_type)
                        .map(|t: Decoded<T>| Hateoas::from(t.into()))
                })
                .map_err(|e| e.into())
                .map(|mut t| {
                    *t.metadata_mut().headers_mut() = Some(headers.uncommon_extract());
                    *t.status_mut().headers_mut() = Some(headers);
                    t
                })
        })
    }
}

impl From<axum_core::extract::rejection::BytesRejection> for Hateoas<()> {
    fn from(e: axum_core::extract::rejection::BytesRejection) -> Self {
        Hateoas::BAD_REQUEST(None, Some("Corrupted payload".to_string()))
    }
}

impl From<simple_serde::Error> for Hateoas<()> {
    fn from(e: simple_serde::Error) -> Self {
        match e {
            Error::Infallible => {
                Hateoas::INTERNAL_SERVER_ERROR(None, Some("How did you end up here?".to_string()))
            }
            Error::ByteToUTF8ConversionFailure(_) => Hateoas::BAD_REQUEST(
                None,
                Some("Payload data not readable into UTF8".to_string()),
            ),
            Error::UnknownContentTypeMatchFromStr(_) => {
                Hateoas::BAD_REQUEST(None, Some("Unknown content type given".to_string()))
            }
            Error::BsonSerializationFailure(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("BSON processing error: {}", e)))
            }
            Error::BsonDeserializationFailure(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("BSON processing error: {}", e)))
            }
            Error::CborFailure(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("CBOR processing error: {}", e)))
            }
            Error::FlexBuffersSerializationFailure(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("Flexbuffer processing error: {}", e)))
            }
            Error::FlexBuffersDeserializationFailure(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("Flexbuffer processing error: {}", e)))
            }
            Error::JsonError(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("JSON processing error: {}", e)))
            }
            Error::Json5Error(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("JSON5 processing error: {}", e)))
            }
            Error::LexprError(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("Lexpr processing error: {}", e)))
            }
            Error::MessagePackEncodeError(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("Message Pack processing error: {}", e)))
            }
            Error::MessagePackDecodeError(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("Message Pack processing error: {}", e)))
            }
            Error::PickleError(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("Pickle processing error: {}", e)))
            }
            Error::PostcardError(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("Postcard processing error: {}", e)))
            }
            Error::RonError(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("Ron processing error: {}", e)))
            }
            Error::TomlSerializationFailure(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("TOML processing error: {}", e)))
            }
            Error::TomlDeserializationFailure(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("TOML processing error: {}", e)))
            }
            Error::UrlEncodingFailure(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("URL Encoded processing error: {:?}", e)))
            }
            Error::YamlError(e) => {
                Hateoas::BAD_REQUEST(None, Some(format!("YAML processing error: {}", e)))
            }
            Error::TypeDoesNotSupportSerialization(_) => Hateoas::BAD_REQUEST(
                None,
                Some(format!(
                    "Payload datatype does not support De-/Serialization"
                )),
            ),
            Error::FailedConvertingHeaderValueToContentType(_) => Hateoas::BAD_REQUEST(
                None,
                Some(format!("Header for typecontent is not supported")),
            ),
            Error::InvalidHeaderValue(_) => Hateoas::BAD_REQUEST(
                None,
                Some(format!("Header for typecontent is not supported")),
            ),
            _ => Hateoas::BAD_REQUEST(None, Some(format!("Unknown payload content"))),
        }
    }
}
