use crate::serde::Serialize;
use crate::RelLinkCollection;
use railsgun::OptionsExtended;
use std::ops::{Deref, DerefMut};

#[derive(Serialize, PartialEq, Debug)]
pub struct Content<T: Serialize> {
    content: Option<T>,
    rel: Option<RelLinkCollection>,
}
impl<T: Serialize> Content<T> {
    /// Setting the content on the Content container
    ///
    /// ```
    /// use hateoas::Content;
    /// let mut ctn_with_content = Content::new(());
    ///
    /// assert_eq!(ctn_with_content.has_content(), true);
    /// assert_eq!(ctn_with_content.content(), &Some(()));
    /// ```
    pub fn new(content: T) -> Self {
        Content {
            content: Some(content),
            rel: None,
        }
    }

    /// Checking if the content has any information in it, eg. is not none
    ///
    /// ```
    /// use hateoas::Content;
    /// let ctn: Content<()> = Content::default();
    /// let mut ctn_with_content = Content::new(Some(()));
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
    /// assert_eq!(ctn.content_mut(), &None);
    ///
    /// let mut_ctn = ctn.content_mut();
    /// *(mut_ctn) = Some("bar".to_string());
    ///
    /// assert_eq!(ctn.content(), &Some("bar".to_string()));
    /// ```
    pub fn content_mut(&mut self) -> &mut Option<T> {
        &mut self.content
    }

    /// Getting a reference of the current spec content
    /// This will get a Option<T> of the current contents spec piece
    /// ```
    /// use hateoas::{Content, RelLinkCollection};
    /// let mut ctn = Content::default();
    ///
    /// assert_eq!(ctn.content(), &None);
    ///
    /// *(ctn.content_mut()) = Some(());
    ///
    /// assert_eq!(ctn.content(), &Some(()))
    /// ```
    pub fn content(&self) -> &Option<T> {
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
impl<T: Serialize> Deref for Content<T> {
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
impl<T: Serialize> DerefMut for Content<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl<T: Serialize> From<T> for Content<T> {
    fn from(t: T) -> Self {
        Content::new(t)
    }
}
