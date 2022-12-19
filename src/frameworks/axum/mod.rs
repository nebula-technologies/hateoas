use crate::header::HeaderKey;
use crate::{status_code, Hateoas, HateoasResource, HeaderMap};
use axum::async_trait;
use axum::body::HttpBody;
use axum::extract::FromRequest;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum_core::BoxError;
use bytes::Bytes;
use http::{header, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use simple_serde::{Decoded, Error, SimpleDecoder, SimpleEncoder};
use tracing::{event, span, Level};

const RESPONSE_MISSING_STATUS_CODE: &'static str = r##"{"apiVersion": "v1","kind": "Error","spec": null,"status": {"message":"Missing status code response","http_status_code": 500}}"##;
const RESPONSE_MISSING_CONTENT_TYPE: &'static str = r##"{"apiVersion": "v1","kind": "Error","spec": null,"status": {"message":"Missing status code response","http_status_code": 500}}"##;

#[cfg(test)]
pub mod test_helpers;

impl<T: Serialize + HateoasResource + Clone> IntoResponse for Hateoas<T> {
    fn into_response(self) -> Response {
        span!(Level::TRACE, "Hateoas Into Response");
        let tmp_response = Response::new(());
        let request_headers = tmp_response.extensions().get::<HeaderMap>();
        event!(Level::TRACE, "extracting http status code");
        let code: http::StatusCode = self
            .status()
            .and_then(|t| t.http_status_code.clone())
            .unwrap_or(status_code::OK)
            .into();
        event!(Level::TRACE, "Extracting Headers from status object");
        let headers = self
            .status()
            .and_then(|t| t.header.clone())
            .and_then(|t| http::HeaderMap::try_from(t).ok())
            .unwrap_or(http::HeaderMap::new());
        event!(
            Level::TRACE,
            "Extracting Accept content type from request extension"
        );
        let accept_content_type = request_headers
            .and_then(|t| t.get(&HeaderKey::Accept).map(|t| t.to_string()))
            .or_else(|| {
                event!(Level::WARN, "Failed to find Accept key in request header extension, falling back to custom header from status");
                headers.get(&HeaderKey::Accept.to_string()).and_then(|t| t.to_str().ok().map(|t|t.to_string()))
            })
            .unwrap_or("application/json".to_string());

        self.encode(accept_content_type)
            .map(|t| {
                let mut response = (code, t.to_vec()).into_response();
                *response.headers_mut() = headers.clone();
                response
            })
            .unwrap_or((StatusCode::INTERNAL_SERVER_ERROR, "").into_response())
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

    async fn from_request(mut req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        span!(Level::Trace, "Extracting Hateoas object from request");
        let headers = HeaderMap::from(req.headers());
        req.extensions_mut().insert(headers.clone());
        let content_type = headers
            .get_first(&HeaderKey::ContentType)
            .map(|t| t.to_owned())
            .ok_or(Hateoas::UNSUPPORTED_MEDIA_TYPE(
                None,
                Some("Content-Type not found in request".to_string()),
            ))?;

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
    use crate::Hateoas;
    use axum::body::HttpBody;
    use axum::extract::FromRequest;
    use axum::routing::{delete, get, post};
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use axum::{Json, Router};
    use http::Method;
    use serde::Deserialize;
    use serde_json::{json, Value};
    use std::net::{SocketAddr, TcpListener};
    use std::ops::Deref;
    use tower::Service; // for `call`
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[derive(Clone, Debug, Deserialize, Serialize)]
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
        Router::new()
            .route(
                "/",
                post(|input: Hateoas<Input>| async move {
                    let hateoas = Hateoas::from(input);
                    let h = hateoas
                        .encode("application/x-ron")
                        .unwrap()
                        .deref()
                        .to_owned();
                    println!("Output: {:#?}", String::from_utf8(h).unwrap());
                    hateoas
                }),
            )
            .route(
                "/",
                get(|| async {
                    let hateoas = Hateoas::from("hello".to_string());
                    let h = hateoas
                        .encode("application/x-ron")
                        .unwrap()
                        .deref()
                        .to_owned();
                    println!("Output: {:#?}", String::from_utf8(h).unwrap());
                    hateoas
                }),
            )
    }

    #[tokio::test]
    async fn deserialize_body() {
        let app = app();

        let mut response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/")
                    .header("Accept", "application/x-ron")
                    .header("Content-Type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"foo":"bar"})).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let data = response.data().await.unwrap().unwrap();
        let t = String::from_utf8(data.to_vec()).unwrap();
        println!("{:?}", t);

        let hateoas = serde_json::from_str::<Hateoas<()>>(&t);
        println!("{:?}", hateoas);

        println!(
            "{}",
            serde_json::to_string(&Hateoas::<()>::INTERNAL_SERVER_ERROR(None, None)).unwrap()
        );
    }
}
