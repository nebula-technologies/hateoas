use crate::header::HeaderMap;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Metadata {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) annotations: HashMap<String, String>,
    pub(crate) signature: Option<String>,
    pub(crate) header: HeaderMap,
}
/// # Metadata
/// Metadata is a collection of alternative information send/recieved from the system
///
/// ## Adding annotation
/// ```
/// use hateoas::{Metadata};
///
/// let mut metadata = Metadata::default();
///
/// metadata.add_annotation("foo", "bar");
/// ```
impl Metadata {
    /// # Metadata - adding annotations
    ///
    /// ```
    /// use hateoas::{Metadata};
    ///
    /// let mut metadata = Metadata::default();
    ///
    /// metadata.add_annotation("foo", "bar");
    /// ```
    pub fn add_annotation(&mut self, key: &str, value: &str) {
        self.annotations.insert(key.to_string(), value.to_string());
    }
}
