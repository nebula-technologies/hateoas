extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde_with;
#[cfg(feature = "actix")]
extern crate actix_web;
#[cfg(feature = "axum")]
extern crate axum;
extern crate bytes;
#[cfg(feature = "actix")]
extern crate futures_core;
#[cfg(any(feature = "simple_serde", future = "axum"))]
extern crate simple_serde;

mod content;
mod frameworks;
mod hateoas;
mod header;
mod http_method;
mod metadata;
mod rel;
mod resource_trait;
mod status;

pub use crate::hateoas::Hateoas;
pub use content::Content;
pub use header::{HeaderMap, HeaderValue};
pub use http_method::HttpMethod;
pub use metadata::Metadata;
pub use rel::rel_link::RelLink;
pub use rel::rel_link_collection::RelLinkCollection;
pub use resource_trait::{AsHateoasResponse, HateoasResource, ToHateoasResponse};
use serde::de::DeserializeOwned;
use serde::Serialize;
pub use status::Status;
use std::ops::{Deref, DerefMut};
