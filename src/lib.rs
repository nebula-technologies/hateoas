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
    use crate::{Content, Hateoas, RelLinkCollection};

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
