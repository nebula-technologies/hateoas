use crate::{Hateoas, HateoasResource};
use axum::extract::FromRequest;
use axum::http::Request;
use axum::response::{IntoResponse, Response, ResponseParts};
use axum_core::extract::FromRequestParts;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;

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
    B: Send + 'static,
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
            req.headers();

            Ok(Hateoas::OK(None, None))
        })
    }
}
