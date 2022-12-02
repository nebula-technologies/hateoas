use crate::{Content, Metadata, Status};

pub trait Hateoas<T> {
    /// ## New Hateoas.
    /// this will create a new instance of Hateoas that will make it easier to crate API replyes for services.
    ///
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let new_hateoas_response: Hateoas<String> = Hateoas::new(None, None, None);
    ///
    /// assert_eq!(new_hateoas_response, Hateoas::default() as Hateoas<String>);
    /// assert_eq!(None, new_hateoas_response.spec());
    /// assert_eq!(None, new_hateoas_response.status());
    /// assert_eq!(None, new_hateoas_response.metadata());
    /// assert_eq!(Some(&"String".to_string()), new_hateoas_response.kind());
    /// assert_eq!(Some(&"hateoas.io/0.0.1".to_string()), new_hateoas_response.api_version());
    /// ```
    fn new(spec: Option<Content<T>>, metadata: Option<Metadata>, status: Option<Status>) -> Self;
    /// ## Getting the kind of the resource
    /// This will return the kind of the resource requested.
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let new_hateoas_response: Hateoas<String> = Hateoas::new(None, None, None);
    ///
    /// assert_eq!(Some(&"String".to_string()), new_hateoas_response.kind());
    /// ```
    fn kind(&self) -> Option<&String>;
    /// ## Getting the api version of the resource
    /// This will get the API version of the resource, this will help as API expands and there are
    /// multiple versions on the system.
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let new_hateoas_response: Hateoas<String> = Hateoas::new(None, None, None);
    ///
    /// assert_eq!(Some(&"hateoas.io/0.0.1".to_string()), new_hateoas_response.api_version());
    /// ```
    fn api_version(&self) -> Option<&String>;
    /// ## Getter for the metadata property
    /// This will get the metadata, if there is no metadata in the system, metadata will no be
    /// initialized, in comparison to the setter `metadata_mut` that will initialize the metadata
    /// property with the metadata object.
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let hateoas: Hateoas<()> = Hateoas::default();
    /// let metadata = hateoas.metadata();
    ///
    /// assert_eq!(None, metadata)
    /// ```
    fn metadata(&self) -> Option<&Metadata>;
    /// Getting the metadata from the response.
    /// By default metadata is not initialized and will be initialized upon usage.
    /// ```
    /// use hateoas::{Hateoas, Metadata};
    ///
    /// let mut response: Hateoas<()> = Hateoas::default();
    /// let mut metadata = Metadata::default();
    ///
    /// assert_eq!(Some(&mut metadata), response.metadata_mut());
    /// ```
    fn metadata_mut(&mut self) -> Option<&mut Metadata>;

    /// ## Getter for the status property
    /// This will get the status, if there is no status in the system, status will no be
    /// initialized, in comparison to the setter `status_mut` that will initialize the status
    /// property with the status object.
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let hateoas: Hateoas<()> = Hateoas::default();
    /// let status = hateoas.status();
    ///
    /// assert_eq!(None, status)
    /// ```
    fn status(&self) -> Option<&Status>;
    /// Get The status information from the response,
    /// If this is not initialized it will be initialized and returned.
    /// ```
    /// use hateoas::{Hateoas, Status};
    ///
    /// let mut response: Hateoas<()> = Hateoas::default();
    ///
    /// let mut status = response.status_mut();
    /// assert_eq!(Some(&mut Status::default()), status)
    /// ```
    fn status_mut(&mut self) -> Option<&mut Status>;

    /// ## Getter for the spec property
    /// This will get the spec, if there is no spec in the system, spec will no be
    /// initialized, in comparison to the setter `spec_mut` that will initialize the spec
    /// property with the spec object.
    /// ```
    /// use hateoas::Hateoas;
    ///
    /// let hateoas: Hateoas<()> = Hateoas::default();
    /// let spec = hateoas.spec();
    ///
    /// assert_eq!(None, spec)
    /// ```
    fn spec(&self) -> Option<&Content<T>>;

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
    /// assert_eq!(Some(&mut Content::default()), spec)
    /// ```
    fn spec_mut(&mut self) -> Option<&mut Content<T>>;

    fn content(&self) -> Option<&T>;

    fn content_mut(&mut self) -> Option<&mut T>;
}
