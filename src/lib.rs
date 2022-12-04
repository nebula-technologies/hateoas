#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_with;
extern crate bytes;

mod content;
mod hateoas;
mod header;
mod http_method;
mod metadata;
mod rel;
mod resource_trait;
mod status;

pub use crate::hateoas::Hateoas;
pub use content::Content;
pub use http_method::HttpMethod;
pub use metadata::Metadata;
pub use rel::rel_link::RelLink;
pub use rel::rel_link_collection::RelLinkCollection;
pub use resource_trait::{AsHateoasResponse, HateoasResource, ToHateoasResponse};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
pub use status::Status;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub struct Payload<T: Serialize + DeserializeOwned>(T);

impl<T: Serialize + DeserializeOwned> Payload<T> {
    pub fn new_hateoas<U: Serialize + DeserializeOwned + Default + HateoasResource>(
        spec: Option<Content<U>>,
        metadata: Option<Metadata>,
        status: Option<Status>,
    ) -> Payload<Hateoas<U>> {
        Payload(Hateoas::new(spec, metadata, status))
    }

    pub fn new(val: T) -> Self {
        Payload(val)
    }
}

impl<T: Serialize + DeserializeOwned> Deref for Payload<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Serialize + DeserializeOwned> DerefMut for Payload<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use crate::{Content, Hateoas, HateoasResource, RelLinkCollection};

    #[derive(Serialize, Deserialize)]
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

    impl HateoasResource for RubberBullet {
        const KIND: &'static str = "";
        const VERSION: &'static str = "";
        const GROUP: &'static str = "";
        const URL_PATH_SEGMENT: &'static str = "";
    }

    const RUBBER_BULLET_SER: &str = r#"{
      "apiVersion": "/",
      "kind": "",
      "metadata": null,
      "spec": {
        "content": {
          "name": "Rubber Bullet",
          "title": "The Bullet",
          "chapter": "A Rubber Bullet Hurts"
        },
        "rel": null
      },
      "status": {
        "message": "OK",
        "code": null,
        "http_status_code": 200,
        "session": null
      }
    }"#;

    #[test]
    pub fn serialize_test() {
        let rubber_bullet = RubberBullet {
            name: "Rubber Bullet".to_string(),
            title: "The Bullet".to_string(),
            chapter: "A Rubber Bullet Hurts".to_string(),
        };

        let response = Hateoas::OK(Some(rubber_bullet));

        let response_ser: serde_json::Value = serde_json::to_value(&response).unwrap();

        println!("{:#?}", response_ser);
    }
    #[test]
    pub fn deserialize_test() {
        let response_ser: serde_json::Value = serde_json::from_str(RUBBER_BULLET_SER).unwrap();

        println!("{:#?}", response_ser);
    }

    // #[test]
    // fn default_response_test() {
    //     let response: Response<String> = Response {
    //         content: None,
    //         metadata: Default::default(),
    //     };
    //     let response_ser = response.encode("yaml");
    //
    //     // println!("{}", response_ser);
    //     // assert_eq!()
    // }

    #[test]
    fn test_content_rel() {
        let mut content: Content<()> = Content::default();
        let rel = content.rel();
        assert_eq!(&mut RelLinkCollection::default(), rel);
    }

    #[test]
    fn test_get_spec_on_none() {
        let mut response: Hateoas<String> = Hateoas::default();

        // Here spec will be None at initialization time.
        // at [Response.spec()] Spec will be initialized and returned.

        let mut spec = response.spec_mut();
        assert_eq!(Some(&mut Content::default()), spec)
    }
}
