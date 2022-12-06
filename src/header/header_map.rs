use crate::header::HeaderValue;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HeaderMap(HashMap<String, HeaderValue>);

impl HeaderMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Append a key/value pair to the headers
    ///
    /// ```rust
    /// use hateoas::{HeaderMap};
    ///
    /// let mut map = HeaderMap::new();
    /// map.set("Content-Type", "application/json");
    ///
    /// assert_eq!(HeaderMap::from(("Content-Type", "application/json")), map);
    /// ```
    pub fn set<V: Into<HeaderValue>>(&mut self, key: &str, value: V) -> &Self {
        self.0.insert(key.to_string(), value.into());
        self
    }

    /// Append a key/value pair to the headers
    ///
    /// ```rust
    /// use hateoas::{HeaderMap};
    ///
    /// let mut map = HeaderMap::new();
    /// map.append("Content-Type", "application/json");
    ///
    /// assert_eq!(HeaderMap::from(("Content-Type", "application/json")), map);
    ///
    /// map.append("Content-Type", "application/x-yaml");
    /// assert_eq!(HeaderMap::from(("Content-Type", vec!["application/json","application/x-yaml"])), map);
    /// ```
    pub fn append<'a, V: Into<HeaderValue>>(&mut self, key: &str, value: V) -> &Self {
        if let Some(values) = self.0.get_mut(key) {
            values.append(value);
        } else {
            self.0.insert(key.to_string(), value.into());
        }
        self
    }
}

impl Default for HeaderMap {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for HeaderMap {
    type Target = HashMap<String, HeaderValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HeaderMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<(&str, &str)> for HeaderMap {
    fn from(t: (&str, &str)) -> Self {
        let mut map = HeaderMap::new();
        map.append(t.0, t.1);
        map
    }
}
impl From<(&str, Vec<&str>)> for HeaderMap {
    fn from(t: (&str, Vec<&str>)) -> Self {
        let mut map = HeaderMap::new();
        map.append(t.0, t.1);
        map
    }
}

impl From<(&str, String)> for HeaderMap {
    fn from(t: (&str, String)) -> Self {
        let mut map = HeaderMap::new();
        map.append(t.0, t.1);
        map
    }
}
impl From<(&str, Vec<String>)> for HeaderMap {
    fn from(t: (&str, Vec<String>)) -> Self {
        let mut map = HeaderMap::new();
        map.append(t.0, t.1);
        map
    }
}

impl From<Vec<(&str, &str)>> for HeaderMap {
    fn from(t: Vec<(&str, &str)>) -> Self {
        let mut map = HeaderMap::new();
        for (key, val) in t {
            map.set(key, val);
        }
        map
    }
}
impl From<Vec<(&str, Vec<&str>)>> for HeaderMap {
    fn from(t: Vec<(&str, Vec<&str>)>) -> Self {
        let mut map = HeaderMap::new();
        for (key, val) in t {
            map.set(key, val);
        }
        map
    }
}

impl From<Option<()>> for HeaderMap {
    fn from(_: Option<()>) -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::HeaderMap;

    #[test]
    pub fn append_header() {
        let mut map = HeaderMap::new();
        map.append("Content-Type", "application/json");

        assert_eq!(HeaderMap::from(("Content-Type", "application/json")), map);
    }
}
