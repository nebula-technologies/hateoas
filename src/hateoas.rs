use crate::resource_trait::HateoasResource;
use crate::serde::Serialize;
use crate::{Content, Metadata, Status};
use railsgun::OptionsExtended;

#[derive(Serialize)]
pub struct HateoasResponse<T: Serialize + HateoasResource> {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub metadata: Option<Metadata>,
    pub spec: Option<Content<T>>,
    pub status: Option<Status>,
}

impl<T: Serialize + Default + HateoasResource> HateoasResponse<T> {
    /// Getting the metadata from the response.
    /// By default metadata is not initialized and will be initialized upon usage.
    /// ```
    /// use hateoas::{HateoasResponse, Metadata};
    ///
    /// let mut response: HateoasResponse<()> = HateoasResponse::default();
    /// let mut metadata = Metadata::default();
    ///
    /// assert_eq!(&mut metadata, response.metadata());
    /// ```
    pub fn metadata(&mut self) -> &mut Metadata {
        self.metadata.get_or_insert_default()
    }

    /// Get The status information from the response,
    /// If this is not initialized it will be initialized and returned.
    /// ```
    /// use hateoas::{HateoasResponse, Status};
    ///
    /// let mut response: HateoasResponse<()> = HateoasResponse::default();
    ///
    /// let mut status = response.status();
    /// assert_eq!(&mut Status::default(), status)
    /// ```
    pub fn status(&mut self) -> &mut Status {
        self.status.get_or_insert_default()
    }

    /// Get the spec information form the Response payload
    ///
    /// ```
    /// use hateoas::{Content, HateoasResponse};
    /// let mut response: HateoasResponse<String> = HateoasResponse::default();
    ///
    /// // Here spec will be None at initialization time.
    /// // at [Response.spec()] Spec will be initialized and returned.
    ///
    /// let mut spec = response.spec();
    /// assert_eq!(&mut Content::default(), spec)
    /// ```
    pub fn spec(&mut self) -> &mut Content<T> {
        self.spec.get_or_insert_default()
    }
}

impl<T: Serialize + HateoasResource> Default for HateoasResponse<T> {
    fn default() -> Self {
        HateoasResponse {
            api_version: "".to_string(),
            kind: "".to_string(),
            metadata: None,
            spec: None,
            status: None,
        }
    }
}
