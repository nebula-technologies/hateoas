pub mod body;
pub mod error;
pub mod future;

use crate::frameworks::actix::error::ActixError;
use crate::frameworks::actix::future::PayloadFuture;
use crate::frameworks::payload_control::{DebuggableAny, PayloadControl};
use crate::Payload;
use actix_web::body::BoxBody;
use actix_web::dev::Response;
use actix_web::http::StatusCode;
use actix_web::{FromRequest, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use simple_serde::ContentType;

impl<T> FromRequest for Payload<T>
where
    T: DeserializeOwned + PayloadControl,
{
    type Error = ActixError;
    type Future = PayloadFuture<T, T, Payload<T>>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut actix_http::Payload) -> Self::Future {
        PayloadFuture::new(req.clone(), payload)
    }
}

impl<T: Serialize> Responder for Payload<T> {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut content_type_collection = req
            .headers()
            .get_all("Accept")
            .filter_map(|h| ContentType::try_from(h).ok())
            .collect::<Vec<ContentType>>();
        if content_type_collection.is_empty() {
            content_type_collection = vec![ContentType::Json];
        }
        match self {
            Ricksponse::Data {
                data, http_code, ..
            } => {
                let response_code = match http_code {
                    Some(code) => StatusCode::from_u16(code).unwrap_or(StatusCode::OK),
                    None => StatusCode::OK,
                };
                content_type_collection.reverse();
                content_type_collection
                    .pop()
                    .ok_or_else(|| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
                    .and_then(|content_type| {
                        data.encode(&content_type)
                            .map(|t| {
                                HttpResponseBuilder::new(response_code)
                                    .content_type(content_type)
                                    .body(t.to_vec())
                            })
                            .map_err(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
                    })
                    .unwrap_or_else(|e| e)
            }
            Ricksponse::Error { http_code, .. } => match http_code {
                Some(code) => HttpResponse::new(
                    StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                ),
                None => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
            },
        }
    }
}

impl<T, E: DebuggableAny> From<Result<T, E>> for Payload<T> {
    fn from(r: Result<T, E>) -> Self {
        let message = if let Err(e) = &r {
            Some(format!("{:?}", e))
        } else {
            None
        };
        match r {
            Err(e) => Self::Error {
                error: Some(Box::new(e)),
                http_code: None,
                message,
            },
            Ok(t) => Self::Data {
                data: t,
                http_code: None,
                message,
            },
        }
    }
}
