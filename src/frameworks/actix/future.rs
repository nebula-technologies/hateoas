use crate::frameworks::actix::body::PayloadBody;
use crate::frameworks::actix::error::ActixError;
use crate::frameworks::payload_control::PayloadControl;
use actix_web::HttpRequest;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct PayloadFuture<O, T, U> {
    pub(crate) _req: Option<HttpRequest>,
    pub(crate) fut: PayloadBody<T, O>,
    pub(crate) phantom: PhantomData<U>,
    pub(crate) phantom_triat: PhantomData<O>,
}

impl<O, T, U> Unpin for PayloadFuture<O, T, U> {}

impl<T: DeserializeOwned, U, O: PayloadControl> PayloadFuture<O, T, U> {
    pub(crate) fn new(r: HttpRequest, p: &mut actix_http::Payload) -> PayloadFuture<O, T, U> {
        PayloadFuture {
            _req: Some(r.clone()),
            fut: PayloadBody::new(r, p),
            phantom: PhantomData::default(),
            phantom_triat: PhantomData::default(),
        }
    }
}

pub struct Payload<T> {
    data: T,
}

impl<T: DeserializeOwned, U: From<Result<T, ActixError>>, O: PayloadControl> Future
    for PayloadFuture<O, T, U>
{
    type Output = Result<U, ActixError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let res = match Pin::new(&mut this.fut).poll(cx) {
            std::task::Poll::Ready(t) => t,
            std::task::Poll::Pending => {
                return std::task::Poll::Pending;
            }
        };
        Poll::Ready(match res {
            Err(err) => Ok(U::from(Err(err.into()) as Result<T, ActixError>)),
            Ok(data) => Ok(U::from(Ok(data) as Result<T, ActixError>)),
        })
    }
}
