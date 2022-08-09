
use crate::RelLink;

/// # RelLinkCollection
///
///
/// ## Adding new data to the collection
/// ```
/// use hateoas::{HttpMethod, RelLink, RelLinkCollection};
///
/// let rel_vec = vec![
///     RelLink::new("foo", "foo", HttpMethod::Get),
///     RelLink::new("bar", "bar", HttpMethod::Get)
/// ];
/// let rlc_check = RelLinkCollection::new(rel_vec);
///
/// let mut rlc = RelLinkCollection::default();
/// rlc.add("foo", RelLink::new("foo", "foo", HttpMethod::Get));
/// rlc.add("bar", RelLink::new("bar", "bar", HttpMethod::Get));
/// ```
///
/// ## Adding new data but data is overwritten
/// ```
/// use hateoas::{HttpMethod, RelLink, RelLinkCollection};
///
/// let rel_vec = vec![
///     RelLink::new("foo", "foo", HttpMethod::Get),
///     RelLink::new("bar", "bar", HttpMethod::Get)
/// ];
/// let mut rlc = RelLinkCollection::new(rel_vec);
///
/// let old_rel = rlc.add("foo", RelLink::new("foo-bar", "foo", HttpMethod::Get));
///
/// assert_eq!(old_rel, Some(("foo", "foo", HttpMethod::Get).into()));
/// ```
///
/// ## Get RelLink from collection
/// ```
/// use hateoas::{HttpMethod, RelLink, RelLinkCollection};
///
/// let rel_vec = vec![
///     RelLink::new("foo", "foo", HttpMethod::Get),
///     RelLink::new("bar", "bar", HttpMethod::Get)
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
///     RelLink::new("foo", "foo", HttpMethod::Get),
///     RelLink::new("bar", "bar", HttpMethod::Get)
/// ];
/// let mut rlc = RelLinkCollection::new(rel_vec);
///
/// let mut rel = rlc.get_mut("foo");
///
/// assert_eq!(rel, Some(&mut ("foo", "foo", HttpMethod::Get).into()));
///
/// rel.map(|t| *t = ("foo-bar", "foo-bar", HttpMethod::Connect).into());
///
/// let updated_rel = rlc.get("foo-bar");
///
/// assert_eq!(updated_rel, Some(&("foo-bar", "foo-bar", HttpMethod::Connect).into()));
/// ```

#[derive(Debug, Serialize, PartialEq, Default)]
pub struct RelLinkCollection(Vec<RelLink>);

impl RelLinkCollection {
    pub fn new(v_rel: Vec<RelLink>) -> Self {
        RelLinkCollection(v_rel)
    }

    pub fn get(&self, rel: &str) -> Option<&RelLink> {
        self.0.iter().find(|rl| rl.rel == rel)
    }

    pub fn has(&self, rel: &str) -> bool {
        self.get(rel).is_some()
    }

    pub fn get_mut(&mut self, rel: &str) -> Option<&mut RelLink> {
        self.0.iter_mut().find(|rl| rl.rel == rel)
    }

    pub fn add(&mut self, rel: &str, link: RelLink) -> Option<RelLink> {
        let mut new_link = link;
        new_link.rel = rel.to_string();
        let mut old_link = None;
        if let Some(found_rel) = self.get_mut(rel) {
            old_link = Some(found_rel.clone());
            *found_rel = new_link;
        } else {
            self.0.push(new_link)
        }
        old_link
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
