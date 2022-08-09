use std::collections::HashMap;

pub trait HateoasResource {
    const KIND: &'static str;
    const VERSION: &'static str;
    const GROUP: &'static str;
    const URL_PATH_SEGMENT: &'static str;
}

impl HateoasResource for String {
    const KIND: &'static str = "String";
    const VERSION: &'static str = "0.0.1-beta.1";
    const GROUP: &'static str = "defaults";
    const URL_PATH_SEGMENT: &'static str = "string";
}

impl HateoasResource for u16 {
    const KIND: &'static str = "U16";
    const VERSION: &'static str = "0.0.1-beta.1";
    const GROUP: &'static str = "defaults";
    const URL_PATH_SEGMENT: &'static str = "u16";
}

impl HateoasResource for u32 {
    const KIND: &'static str = "U32";
    const VERSION: &'static str = "0.0.1-beta.1";
    const GROUP: &'static str = "defaults";
    const URL_PATH_SEGMENT: &'static str = "u32";
}

impl HateoasResource for () {
    const KIND: &'static str = "Void";
    const VERSION: &'static str = "0.0.1-beta.1";
    const GROUP: &'static str = "defaults";
    const URL_PATH_SEGMENT: &'static str = "void";
}

impl HateoasResource for HashMap<String, String> {
    const KIND: &'static str = "HashMap";
    const VERSION: &'static str = "0.0.1-beta.1";
    const GROUP: &'static str = "defaults";
    const URL_PATH_SEGMENT: &'static str = "hashmap";
}
