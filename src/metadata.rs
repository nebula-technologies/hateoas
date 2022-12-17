use crate::header::HeaderMap;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Metadata {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) annotations: Option<HashMap<String, String>>,
    pub(crate) signature: Option<String>,
    pub(crate) header: Option<HeaderMap>,
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
    /// # Create a new metadata Object
    /// this is for creating a new metadata object where everything is None.
    ///
    pub fn new(
        id: Option<String>,
        name: Option<String>,
        annotations: Option<HashMap<String, String>>,
        signature: Option<String>,
        header: Option<HeaderMap>,
    ) -> Self {
        Self {
            id,
            name,
            annotations,
            signature,
            header,
        }
    }

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
        self.annotations
            .get_or_insert(Default::default())
            .insert(key.to_string(), value.to_string());
    }

    /// ## Getter for the headers
    ///
    /// ```
    /// use hateoas::{Metadata, Status};
    ///
    /// let uuid = uuid::Uuid::new_v4();
    /// let mut metadata = Metadata::new(None, None, None, None, None);
    ///
    /// assert_eq!(metadata.headers(), &None);
    /// ```
    pub fn headers(&self) -> &Option<HeaderMap> {
        &self.header
    }

    /// ## Getter for mutable headers
    ///
    /// ```
    /// use hateoas::{HeaderMap, Metadata};
    ///
    /// let uuid = uuid::Uuid::new_v4();
    /// let mut metadata = Metadata::default();
    ///
    /// metadata
    ///     .headers_mut()
    ///     .get_or_insert(Default::default())
    ///     .append("test", uuid.to_string());
    /// metadata
    ///     .headers_mut()
    ///     .as_mut()
    ///     .map(|t| t.append("test2", uuid.to_string()));
    ///
    /// let test_headers: HeaderMap = vec![
    ///     ("test", uuid.to_string().as_str()),
    ///     ("test2", uuid.to_string().as_str()),
    /// ]
    /// .into();
    /// assert_eq!(metadata.headers(), &Some(test_headers));
    /// ```
    pub fn headers_mut(&mut self) -> &mut Option<HeaderMap> {
        &mut self.header
    }
}

#[cfg(test)]
pub mod test {
    use crate::{HeaderMap, Metadata, Status};

    #[test]
    pub fn test_headers_mut() {
        let uuid = uuid::Uuid::new_v4();
        let mut metadata = Metadata::default();

        metadata
            .headers_mut()
            .get_or_insert(Default::default())
            .append("test", uuid.to_string());
        metadata
            .headers_mut()
            .as_mut()
            .map(|t| t.append("test2", uuid.to_string()));

        let test_headers: HeaderMap = vec![
            ("test", uuid.to_string().as_str()),
            ("test2", uuid.to_string().as_str()),
        ]
        .into();
        assert_eq!(metadata.headers(), &Some(test_headers));
    }
}
