pub mod body;
pub mod error;
pub mod future;

use crate::frameworks::actix::error::ActixError;
use crate::frameworks::actix::future::PayloadFuture;
use crate::frameworks::payload_control::PayloadControl;
use crate::{Hateoas, HateoasResource};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{FromRequest, HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use simple_serde::{ContentType, SimpleEncoder};

impl<T> FromRequest for Hateoas<T>
where
    T: DeserializeOwned + PayloadControl + HateoasResource + Clone,
{
    type Error = ActixError;
    type Future = PayloadFuture<T, T>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut actix_http::Payload) -> Self::Future {
        PayloadFuture::new(req.clone(), payload)
    }
}

impl<T> Responder for Hateoas<T>
where
    T: HateoasResource + Serialize + Clone,
{
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

        let response_code = StatusCode::OK;
        content_type_collection.reverse();
        content_type_collection
            .pop()
            .ok_or_else(|| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
            .and_then(|content_type| {
                self.encode(&content_type)
                    .map(|t| {
                        HttpResponseBuilder::new(response_code)
                            .content_type(content_type)
                            .body(t.to_vec())
                    })
                    .map_err(|_| HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
            })
            .unwrap_or_else(|e| e)
    }
}

impl<T> From<ActixError> for Hateoas<T>
where
    T: HateoasResource + Clone,
{
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

impl<T> From<Result<T, ActixError>> for Hateoas<T>
where
    T: HateoasResource + Clone,
{
    fn from(t: Result<T, ActixError>) -> Self {
        match t {
            Ok(t) => t.into(),
            Err(e) => e.into(),
        }
    }
}
impl<T> From<Result<Hateoas<T>, ActixError>> for Hateoas<T>
where
    T: HateoasResource + Clone,
{
    fn from(t: Result<Hateoas<T>, ActixError>) -> Self {
        match t {
            Ok(t) => t,
            Err(e) => e.into(),
        }
    }
}

// impl<T, E: DebuggableAny> From<Result<T, E>> for Payload<T> {
//     fn from(r: Result<T, E>) -> Self {
//         let message = if let Err(e) = &r {
//             Some(format!("{:?}", e))
//         } else {
//             None
//         };
//         match r {
//             Err(e) => Self::Error {
//                 error: Some(Box::new(e)),
//                 http_code: None,
//                 message,
//             },
//             Ok(t) => Self::Data {
//                 data: t,
//                 http_code: None,
//                 message,
//             },
//         }
//     }
// }

#[cfg(test)]
mod test {
    use crate::hateoas::Hateoas;
    use actix_web::{http::header, test, web, App};
    use serde_json;
    use std::ops::Deref;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

    impl crate::HateoasResource for RubberBullet {
        const KIND: &'static str = "";
        const VERSION: &'static str = "";
        const GROUP: &'static str = "";
        const URL_PATH_SEGMENT: &'static str = "";
    }

    const RICKSPONSE_1: &str = r##"


    "##;

    #[actix_web::test]
    async fn test_hateoas_string() {
        let app =
            test::init_service(App::new().service(web::resource("/index.html").route(
                web::post().to(|| async { Hateoas::OK(Some("welcome!".to_string()), None) }),
            )))
            .await;

        let req = test::TestRequest::post()
            .uri("/index.html")
            .insert_header(header::ContentType::json())
            .to_request();

        let res = test::call_service(&app, req).await;
        let result = test::read_body(res).await;

        let raw_str = std::str::from_utf8(&*result).unwrap();
        println!("{}", raw_str);
        let content = serde_json::from_str::<Hateoas<String>>(raw_str).unwrap();
        println!("{:#?}", content);
        assert_eq!(content, Hateoas::OK(Some("welcome!".to_string()), None));
    }

    #[actix_web::test]
    async fn test_hateoas_rubber_bullet() {
        let response = Hateoas::OK(Some(RubberBullet::default()), None);

        let app =
            test::init_service(App::new().service(web::resource("/index.html").route(
                web::post().to(|| async { Hateoas::OK(Some(RubberBullet::default()), None) }),
            )))
            .await;

        let req = test::TestRequest::post()
            .uri("/index.html")
            .insert_header(header::ContentType::json())
            .to_request();

        let res = test::call_service(&app, req).await;
        let result = test::read_body(res).await;

        let raw_str = std::str::from_utf8(&*result).unwrap();
        println!("{}", raw_str);
        let content = serde_json::from_str::<Hateoas<RubberBullet>>(raw_str).unwrap();
        println!("{:#?}", content);
        assert_eq!(content, response);
    }

    #[actix_web::test]
    async fn test_for_automated_impl_hateoas() {
        let rickhateoas: Hateoas<String> = Hateoas::OK(Some("test".to_string()), None);

        assert_eq!(
            rickhateoas,
            crate::Hateoas::new(
                Some(crate::Content::new("test".to_string())),
                None,
                Some(crate::Status::OK(None))
            )
        );
    }
}
