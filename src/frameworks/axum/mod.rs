use crate::Payload;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

// impl<T: Serialize + DeserializeOwned> IntoResponse for Payload<T> {
//     fn into_response(self) -> Response {}
// }
