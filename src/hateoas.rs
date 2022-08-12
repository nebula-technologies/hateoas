use crate::resource_trait::HateoasResource;
use crate::serde::Serialize;
use crate::{Content, Metadata, Status};
use railsgun::OptionsExtended;

#[derive(Serialize)]
pub struct Hateoas<T: Serialize + HateoasResource> {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub metadata: Option<Metadata>,
    pub spec: Option<Content<T>>,
    pub status: Option<Status>,
}

impl<T: Serialize + Default + HateoasResource> Hateoas<T> {
    /// ## New Hateoas.
    /// this will create a new instance of Hateoas that will make it easier to crate API replyes for services.
    ///
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let new_hateoas_response = Hateoas::new(Some("Hello world!".to_string().into()), None, None);
    ///
    /// ```
    pub fn new(
        spec: Option<Content<T>>,
        metadata: Option<Metadata>,
        status: Option<Status>,
    ) -> Self {
        Self {
            api_version: format!("{}/{}", T::GROUP, T::VERSION),
            kind: format!("{}", T::KIND),
            metadata,
            spec,
            status,
        }
    }

    pub fn metadata(&self) -> &Option<Metadata> {
        &self.metadata
    }
    /// Getting the metadata from the response.
    /// By default metadata is not initialized and will be initialized upon usage.
    /// ```
    /// use hateoas::{Hateoas, Metadata};
    ///
    /// let mut response: Hateoas<()> = Hateoas::default();
    /// let mut metadata = Metadata::default();
    ///
    /// assert_eq!(&mut metadata, response.metadata_mut());
    /// ```
    pub fn metadata_mut(&mut self) -> &mut Metadata {
        self.metadata.get_or_insert_default()
    }

    pub fn status(&self) -> &Option<Status> {
        &self.status
    }
    /// Get The status information from the response,
    /// If this is not initialized it will be initialized and returned.
    /// ```
    /// use hateoas::{Hateoas, Status};
    ///
    /// let mut response: Hateoas<()> = Hateoas::default();
    ///
    /// let mut status = response.status_mut();
    /// assert_eq!(&mut Status::default(), status)
    /// ```
    pub fn status_mut(&mut self) -> &mut Status {
        self.status.get_or_insert_default()
    }

    pub fn spec(&self) -> &Option<Content<T>> {
        &self.spec
    }

    /// Get the spec information form the Response payload
    ///
    /// ```
    /// use hateoas::{Content, Hateoas};
    /// let mut response: Hateoas<String> = Hateoas::default();
    ///
    /// // Here spec will be None at initialization time.
    /// // at [Response.spec_mut()] Spec will be initialized and returned.
    ///
    /// let mut spec = response.spec_mut();
    /// assert_eq!(&mut Content::default(), spec)
    /// ```
    pub fn spec_mut(&mut self) -> &mut Content<T> {
        self.spec.get_or_insert_default()
    }
}

impl<T: Serialize + HateoasResource> Default for Hateoas<T> {
    fn default() -> Self {
        Hateoas {
            api_version: "".to_string(),
            kind: "".to_string(),
            metadata: None,
            spec: None,
            status: None,
        }
    }
}
