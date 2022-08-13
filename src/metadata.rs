use std::collections::HashMap;

#[derive(Serialize, PartialEq, Debug, Default)]
pub struct Metadata {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) annotations: HashMap<String, String>,
    pub(crate) signature: Option<String>,
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
