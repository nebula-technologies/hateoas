use crate::RelLink;

/// # RelLinkCollection
///
///
/// ## Adding new data to the collection
/// ```
/// use hateoas::{HttpMethod, RelLink, RelLinkCollection};
///
/// let rel_vec = vec![
///     RelLink::new( "foo","foo", HttpMethod::Get, None),
///     RelLink::new( "bar","bar", HttpMethod::Get, None)
/// ];
/// let rlc_check = RelLinkCollection::new(rel_vec);
///
/// let mut rlc = RelLinkCollection::default();
/// rlc.add(RelLink::new( "foo","foo", HttpMethod::Get, None));
/// rlc.add(RelLink::new( "bar","bar", HttpMethod::Get, None));
/// ```
///
/// ## Adding new data but data is overwritten
/// ```
/// use hateoas::{HttpMethod, RelLink, RelLinkCollection};
///
/// let rel_vec = vec![
///     RelLink::new( "foo","foo", HttpMethod::Get, None),
///     RelLink::new( "bar","bar", HttpMethod::Get, None)
/// ];
/// let mut rlc = RelLinkCollection::new(rel_vec);
///
/// let old_rel = rlc.add(RelLink::new( "foo","foo-bar", HttpMethod::Get, None));
///
/// assert_eq!(old_rel, Some(("foo", "foo", HttpMethod::Get).into()));
/// ```
///
/// ## Get RelLink from collection
/// ```
/// use hateoas::{HttpMethod, RelLink, RelLinkCollection};
///
/// let rel_vec = vec![
///     RelLink::new( "foo","foo", HttpMethod::Get, None),
///     RelLink::new( "bar","bar", HttpMethod::Get, None)
/// ];
/// let mut rlc = RelLinkCollection::new(rel_vec);
///
/// let rel = rlc.get("foo");
///
/// assert_eq!(rel, Some(&("foo", "foo", HttpMethod::Get).into()));
/// ```
///
/// ## Get Mutable RelLink from and updateing it.
/// ```
/// use hateoas::{HttpMethod, RelLink, RelLinkCollection};
///
/// let rel_vec = vec![
///     RelLink::new( "foo","foo", HttpMethod::Get, None),
///     RelLink::new( "bar","bar", HttpMethod::Get, None)
/// ];
/// let mut rlc = RelLinkCollection::new(rel_vec);
///
/// let mut rel = rlc.get_mut("foo");
///
/// assert_eq!(rel, Some(&mut ("foo", "foo", HttpMethod::Get, None).into()));
///
/// rel.map(|t| *t = ("foo-bar", "foo-bar", HttpMethod::Connect, None).into());
///
/// let updated_rel = rlc.get("foo-bar");
///
/// assert_eq!(updated_rel, Some(&("foo-bar", "foo-bar", HttpMethod::Connect, None).into()));
/// ```

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct RelLinkCollection(Vec<RelLink>);

impl RelLinkCollection {
    /// ## Create Collection
    /// Create new Collection with complete Vec, this allows to set all the elements for the collection in one go using a Vec.
    /// ```
    /// use hateoas::{RelLink, RelLinkCollection};
    ///
    /// let collection = RelLinkCollection::new(vec![]);
    /// let vec_col: Vec<RelLink> = collection.into();
    ///
    /// assert_eq!(vec_col, vec![]);
    /// ```
    pub fn new(v_rel: Vec<RelLink>) -> Self {
        RelLinkCollection(v_rel)
    }

    /// ## Get RelLink from Collection
    /// Getting a RelLink from the collection by rel id.
    ///
    /// ```
    /// use hateoas::{RelLinkCollection, HttpMethod, RelLink};
    ///
    /// let collection = RelLinkCollection::new(vec![("rel_id", "/rel_path/", HttpMethod::Get).into()]);
    ///
    /// assert_eq!(collection.get("rel_id"), Some(&RelLink::new("rel_id", "/rel_path/", HttpMethod::Get, None)));
    /// ```
    pub fn get(&self, rel: &str) -> Option<&RelLink> {
        self.0.iter().find(|rl| rl.rel == rel)
    }

    /// ## Get RelLink from Collection
    /// Getting a RelLink from the collection by rel id.
    ///
    /// ```
    /// use hateoas::{RelLinkCollection, HttpMethod, RelLink};
    ///
    /// let mut collection = RelLinkCollection::new(vec![("rel_id", "/rel_path/", HttpMethod::Get, None).into()]);
    /// let mut rel_from_collection = collection.get_mut("rel_id");
    /// rel_from_collection.map(|t| *t = RelLink::new("rel_id_2", "/rel_path_2/", HttpMethod::Connect, None));
    ///
    /// assert_eq!(collection.get("rel_id_2"), Some(&RelLink::new("rel_id_2", "/rel_path_2/", HttpMethod::Connect, None)));
    /// ```
    pub fn get_mut(&mut self, rel: &str) -> Option<&mut RelLink> {
        self.0.iter_mut().find(|rl| rl.rel == rel)
    }

    /// ## Has RelLink in Collection
    /// Checking if a rel_id already exists in the collection.
    ///
    /// ```
    /// use hateoas::{RelLinkCollection, HttpMethod, RelLink};
    ///
    /// let mut collection = RelLinkCollection::new(vec![("rel_id", "/rel_path/", HttpMethod::Get).into()]);
    ///
    /// assert_eq!(collection.has("rel_id"), true);
    /// assert_eq!(collection.has("rel_id_2"), false);
    /// ```
    pub fn has(&self, rel: &str) -> bool {
        self.get(rel).is_some()
    }

    /// ## Add RelLink to the collection
    /// Adding a link to the collection of RelLinks, if the link already exists it will insert the new data and return the old.
    ///
    /// ```
    /// use hateoas::{RelLinkCollection, HttpMethod, RelLink};
    ///
    /// let mut collection = RelLinkCollection::new(vec![("rel_id", "/rel_path/", HttpMethod::Get).into()]);
    /// let old_data = collection.add(("rel_id", "/rel_path_2/", HttpMethod::Connect));
    ///
    /// assert_eq!(old_data, Some(RelLink::new("rel_id", "/rel_path/", HttpMethod::Get, None)));
    /// assert_eq!(collection.get("rel_id"), Some(&RelLink::new("rel_id", "/rel_path_2/", HttpMethod::Connect, None)));
    /// ```
    pub fn add<I: Into<RelLink>>(&mut self, rel: I) -> Option<RelLink> {
        let new_link: RelLink = rel.into();
        let mut old_link = None;
        if let Some(found_rel) = self.get_mut(new_link.rel()) {
            old_link = Some(found_rel.clone());
            *found_rel = new_link;
        } else {
            self.0.push(new_link)
        }
        old_link
    }
}

impl From<RelLinkCollection> for Vec<RelLink> {
    fn from(col: RelLinkCollection) -> Self {
        col.0
    }
}

impl<I: Into<RelLink>> From<I> for RelLinkCollection {
    fn from(r: I) -> Self {
        RelLinkCollection(vec![r.into()])
    }
}

impl<I: Into<RelLink>> From<Vec<I>> for RelLinkCollection {
    fn from(v_rel: Vec<I>) -> Self {
        RelLinkCollection(
            v_rel
                .into_iter()
                .map(|e| e.into())
                .collect::<Vec<RelLink>>(),
        )
    }
}
