use crate::frameworks::actix::body::PayloadBody;
use crate::frameworks::actix::error::ActixError;
use crate::frameworks::payload_control::PayloadControl;
use crate::{Hateoas, HateoasResource};
use actix_web::HttpRequest;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct PayloadFuture<O, T> {
    pub(crate) _req: Option<HttpRequest>,
    pub(crate) fut: PayloadBody<T, O>,
    pub(crate) phantom_triat: PhantomData<O>,
}

impl<O, T> Unpin for PayloadFuture<O, T> {}

impl<T: DeserializeOwned, O: PayloadControl> PayloadFuture<O, T> {
    pub(crate) fn new(r: HttpRequest, p: &mut actix_http::Payload) -> PayloadFuture<O, T> {
        PayloadFuture {
            _req: Some(r.clone()),
            fut: PayloadBody::new(r, p),
            phantom_triat: PhantomData::default(),
        }
    }
}

impl<T: DeserializeOwned + HateoasResource, O: PayloadControl> Future for PayloadFuture<O, T> {
    type Output = Result<Hateoas<T>, ActixError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let res = match Pin::new(&mut this.fut).poll(cx) {
            std::task::Poll::Ready(t) => t,
            std::task::Poll::Pending => {
                return std::task::Poll::Pending;
            }
        };
        Poll::Ready(match res {
            Err(err) => Ok(Hateoas::from(
                Err(err.into()) as Result<Hateoas<T>, ActixError>
            )),
            Ok(data) => Ok(data),
        })
    }
}
