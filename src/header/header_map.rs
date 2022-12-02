use crate::header::HeaderValue;
use bytes::Bytes;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HeaderMap(HashMap<String, HeaderValue>);

impl HeaderMap {
    pub fn insert(&mut self, key: &str, value: HeaderValue) -> &Self {
        self.0.insert(key.to_string(), value);
        self
    }

    pub fn append<'a, B: Into<&'a [u8]>>(&mut self, key: &str, value: B) -> &Self {
        if let Some(values) = self.0.get_mut(key) {
            values.insert(Bytes::from(value.into().to_vec()));
        } else {
            self.0.insert(
                key.to_string(),
                HeaderValue::new_with_value(vec![Bytes::from(value.into().to_vec())]),
            );
        }
        self
    }
}

impl Default for HeaderMap {
    fn default() -> Self {
        Self(HashMap::new())
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
