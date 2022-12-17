use crate::{Hateoas, HateoasResource, HeaderMap};
use axum::async_trait;
use axum::body::HttpBody;
use axum::extract::FromRequest;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum_core::BoxError;
use bytes::Bytes;
use serde::de::DeserializeOwned;
use serde::Serialize;
use simple_serde::{Decoded, Error, SimpleDecoder, SimpleEncoder};
use tracing::{event, span, Level};

#[cfg(test)]
pub mod test_helpers;

impl<T: Serialize + HateoasResource + Clone> IntoResponse for Hateoas<T> {
    fn into_response(self) -> Response {
        let status = self
            .status()
            .as_ref()
            .and_then(|t| t.http_status_code.clone())
            .unwrap_or(crate::status_code::INTERNAL_SERVER_ERROR);
        let headers = self
            .status()
            .as_ref()
            .and_then(|t| t.header.clone())
            .and_then(|t| http::HeaderMap::try_from(t).ok());
        let content_type = headers
            .as_ref()
            .and_then(|t| {
                t.get("Content-Type")
                    .and_then(|t| t.to_str().map(|t| t.to_string()).ok())
            })
            .unwrap_or("".to_string());
        self.encode(content_type)
            .map(|t| {
                let mut response = (http::StatusCode::from(status), t.to_vec()).into_response();
                *response.headers_mut() = headers.unwrap_or(http::HeaderMap::default());
                response
            })
            .unwrap_or(Hateoas::<()>::INTERNAL_SERVER_ERROR(None, None).into_response())
    }
}

// impl<T: DeserializeOwned + HateoasResource, S, B> FromRequest<S, B> for Hateoas<T> {
//     type Rejection = ();
// }
#[async_trait]
impl<S, B, T> FromRequest<S, B> for Hateoas<T>
where
    T: DeserializeOwned + HateoasResource + Clone,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = Hateoas<()>;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::FromRequest;
    use axum::routing::{delete, get, post};
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use axum::{Json, Router};
    use serde::Deserialize;
    use serde_json::{json, Value};
    use std::net::{SocketAddr, TcpListener};
    use tower::Service; // for `call`
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[derive(Clone, Debug, Deserialize)]
    struct Input {
        foo: String,
    }

    impl HateoasResource for Input {
        const KIND: &'static str = "test-input";
        const VERSION: &'static str = "0";
        const GROUP: &'static str = "test";
        const URL_PATH_SEGMENT: &'static str = "/input";
    }

    pub fn app() -> Router {
        Router::new().route(
            "/",
            post(|input: Hateoas<Input>| async { Hateoas::from("hello".to_string()) }),
        )
    }

    #[tokio::test]
    async fn deserialize_body() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"foo":"bar"})).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        println!("{:?}", response)
    }
}
