use crate::Hateoas;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

pub trait HateoasResource {
    /// The constant that defines what type of resources is being send.
    const KIND: &'static str;
    /// The version of the schema used for the expected resource
    const VERSION: &'static str;
    /// What group this resource belongs under. This is usually same as the app domain or package domain.
    const GROUP: &'static str;
    /// This is the current groups/resource url endpoint, this allows for different endpoints for each type/group/version.
    /// an endpoint could be generated as follows:
    /// /[Group]/[Version]/[URL_PATH_SEGMENT]
    const URL_PATH_SEGMENT: &'static str;
}

impl HateoasResource for String {
    const KIND: &'static str = "String";
    const VERSION: &'static str = "0.0.1";
    const GROUP: &'static str = "hateoas.io";
    const URL_PATH_SEGMENT: &'static str = "string";
}

impl HateoasResource for u16 {
    const KIND: &'static str = "U16";
    const VERSION: &'static str = "0.0.1";
    const GROUP: &'static str = "hateoas.io";
    const URL_PATH_SEGMENT: &'static str = "u16";
}

impl HateoasResource for u32 {
    const KIND: &'static str = "U32";
    const VERSION: &'static str = "0.0.1";
    const GROUP: &'static str = "hateoas.io";
    const URL_PATH_SEGMENT: &'static str = "u32";
}

impl HateoasResource for () {
    const KIND: &'static str = "Void";
    const VERSION: &'static str = "0.0.1";
    const GROUP: &'static str = "hateoas.io";
    const URL_PATH_SEGMENT: &'static str = "void";
}

impl HateoasResource for HashMap<String, String> {
    const KIND: &'static str = "HashMap";
    const VERSION: &'static str = "0.0.1";
    const GROUP: &'static str = "hateoas.io";
    const URL_PATH_SEGMENT: &'static str = "hashmap";
}

pub trait ToHateoasResponse<T> {
    fn to_hateoas_response(self) -> T;
}

pub trait AsHateoasResponse<T>
where
    T: Serialize + DeserializeOwned + HateoasResource + Clone,
{
    fn as_response(&mut self) -> &mut Hateoas<T>;
}
