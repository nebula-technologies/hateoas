use crate::serde::Serialize;
use crate::RelLinkCollection;
use railsgun::OptionsExtended;
use std::ops::{Deref, DerefMut};

#[derive(Serialize, PartialEq, Debug)]
pub struct Content<T> {
    content: Option<T>,
    rel: Option<RelLinkCollection>,
}
impl<T> Content<T> {
    /// Setting the content on the Content container
    ///
    /// ```
    /// use hateoas::Content;
    /// let mut ctn_with_content = Content::default();
    /// ctn_with_content.content(());
    ///
    /// assert_eq!(ctn_with_content.has_content(), true);
    /// assert_eq!(ctn_with_content.get_content(), &Some(()));
    /// ```
    pub fn content(&mut self, content: T) {
        self.content = Some(content);
    }

    /// Checking if the content has any information in it, eg. is not none
    ///
    /// ```
    /// use hateoas::Content;
    /// let ctn: Content<()> = Content::default();
    /// let mut ctn_with_content = Content::default();
    /// ctn_with_content.content(());
    ///
    /// assert_eq!(ctn.has_content(), false);
    /// assert_eq!(ctn_with_content.has_content(), true);
    /// ```
    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    /// Getting a mut reference of the current spec content
    /// This will get a Option<&mut T> of the current contents spec piece.
    /// This will allow for modification of the internal content in the spec
    /// ```
    /// use hateoas::{Content};
    /// let mut ctn: Content<String> = Content::default();
    ///
    /// assert_eq!(ctn.get_content(), &None);
    ///
    /// ctn.content("foo".to_string());
    ///
    /// assert_eq!(ctn.get_content(), &Some("foo".to_string()));
    ///
    /// let mut_ctn = ctn.get_mut_content();
    /// mut_ctn.map(|t| *t = "bar".to_string());
    ///
    /// assert_eq!(ctn.get_content(), &Some("bar".to_string()));
    /// ```
    pub fn get_mut_content(&mut self) -> Option<&mut T> {
        self.content.as_mut()
    }

    /// Getting a reference of the current spec content
    /// This will get a Option<T> of the current contents spec piece
    /// ```
    /// use hateoas::{Content, RelLinkCollection};
    /// let mut ctn = Content::default();
    ///
    /// assert_eq!(ctn.get_content(), &None);
    ///
    /// ctn.content(());
    ///
    /// assert_eq!(ctn.get_content(), &Some(()))
    /// ```
    pub fn get_content(&self) -> &Option<T> {
        &self.content
    }

    /// Get the rel even if not set.
    ///
    /// ```
    /// use hateoas::{Content, RelLinkCollection};
    ///
    /// let mut content: Content<()> = Content::default();
    /// let rel = content.rel();
    ///
    /// assert_eq!(rel, &mut RelLinkCollection::default())
    /// ```
    pub fn rel(&mut self) -> &mut RelLinkCollection {
        if self.rel.is_none() {
            self.rel = Some(RelLinkCollection::default());
        }

        self.rel.get_or_insert_default()
    }
}

impl<T: Serialize> Default for Content<T> {
    fn default() -> Self {
        Content {
            content: None,
            rel: None,
        }
    }
}

/// Dereferencing the Internal [T] from the Content object
/// This allows us to better operate on the content itself and use it without having to extract it.
/// ```
/// use std::ops::Deref;
/// use hateoas::Content;
/// let content: Content<()> = Content::default();
/// let content_opt: &Option<()> = content.deref();
///
/// assert_eq!(content_opt, &None);
/// ```
impl<T> Deref for Content<T> {
    type Target = Option<T>;
    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

/// Dereferencing the Internal [T] from the Content object
/// This allows us to better operate on the content itself and use it without having to extract it.
/// ```
/// use std::ops::{Deref, DerefMut};
/// use hateoas::Content;
/// let mut content: Content<()> = Content::default();
/// let content_opt: &Option<()> = content.deref_mut();
///
/// assert_eq!(content_opt, &mut None);
/// ```
impl<T> DerefMut for Content<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
