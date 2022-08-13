#[macro_use]
extern crate serde_derive;
extern crate serde;

mod content;
mod hateoas;
mod metadata;
mod rel_link;
mod rel_link_collection;
mod resource_trait;
mod status;

pub use crate::hateoas::Hateoas;
pub use content::Content;
pub use metadata::Metadata;
pub use rel_link::{HttpMethod, RelLink};
pub use rel_link_collection::RelLinkCollection;
pub use resource_trait::{AsHateoasResponse, HateoasResource, ToHateoasResponse};
pub use status::Status;

#[cfg(test)]
mod test {
    use crate::{Content, Hateoas, HateoasResource, RelLinkCollection};

    #[derive(Serialize, Deserialize, Default)]
    pub struct RubberBullet {
        pub name: String,
        pub title: String,
        pub chapter: String,
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

        let response = Hateoas::OK(rubber_bullet);

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
        assert_eq!(&mut Content::default(), spec)
    }
}
