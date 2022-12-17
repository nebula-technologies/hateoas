use crate::header::{HeaderValue, COMMON_HEADERS};
use bytes::Bytes;
use http::header::{HeaderName, IntoHeaderName};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

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

    pub fn get_first<Q: ?Sized>(&self, key: &Q) -> Option<&Bytes>
    where
        String: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get(key).and_then(|v| v.iter().next())
    }

    pub fn common_extract(&self) -> Self {
        self.extract(COMMON_HEADERS.clone())
    }

    pub fn uncommon_extract(&self) -> Self {
        self.inverse_extract(COMMON_HEADERS.clone())
    }

    pub fn extract<H: Into<HashSet<String>>>(&self, extract: H) -> Self {
        let converted_extract = extract.into();
        let mut extract_headers = HeaderMap::new();
        for (key, value) in &self.0 {
            if converted_extract.contains(key.as_str()) {
                extract_headers.append(key, value.clone());
            }
        }
        extract_headers
    }
    pub fn inverse_extract<H: Into<HashSet<String>>>(&self, extract: H) -> Self {
        let converted_extract = extract.into();
        let mut inv_extract_headers = HeaderMap::new();
        for (key, value) in &self.0 {
            if !converted_extract.contains(key.as_str()) {
                inv_extract_headers.append(key, value.clone());
            }
        }
        inv_extract_headers
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

impl From<http::HeaderMap> for HeaderMap {
    fn from(t: http::HeaderMap) -> Self {
        let mut headers = Self::default();
        for (opt_key, value) in t {
            if let Some(key) = opt_key {
                headers.insert(key.to_string(), value.into());
            }
        }
        headers
    }
}

impl From<&http::HeaderMap> for HeaderMap {
    fn from(t: &http::HeaderMap) -> Self {
        let mut headers = Self::default();
        for (key, value) in t {
            headers.insert(key.to_string(), value.into());
        }
        headers
    }
}

impl TryFrom<HeaderMap> for http::HeaderMap {
    type Error = String;
    fn try_from(h: HeaderMap) -> Result<Self, Self::Error> {
        let mut headers = Self::default();
        for (key, value) in h.0 {
            let header_value_res = http::HeaderValue::try_from(&value);
            let name_res = HeaderName::from_str(key.as_str());
            match (header_value_res, name_res) {
                (Ok(v), Ok(n)) => {
                    headers.append(n, v);
                }
                (Err(e), _) => return Err(e),
                (_, Err(e)) => return Err(e.to_string()),
            }
        }
        Ok(headers)
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
