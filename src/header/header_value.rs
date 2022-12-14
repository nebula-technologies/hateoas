use bytes::Bytes;
use serde::ser::Error;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::str::{from_utf8, Utf8Error};

#[derive(Clone, Debug, PartialEq)]
pub struct HeaderValue(HashSet<String>);

impl HeaderValue {
    pub fn new<'a, B: Into<HeaderValue>>() -> Self {
        Self(HashSet::new())
    }

    pub fn append<V: Into<HeaderValue>>(&mut self, value: V) {
        for v in value.into().0 {
            self.0.insert(v);
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.iter().map(|t| t.clone()).collect::<Vec<String>>()
    }

    pub fn as_vec_u8(&self) -> Vec<u8> {
        self.to_vec().join("").as_bytes().to_vec()
    }
}

impl Default for HeaderValue {
    fn default() -> Self {
        Self(HashSet::new())
    }
}

impl<'de> Deserialize<'de> for HeaderValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor<'de> {
            marker: PhantomData<HeaderValue>,
            lifetime: PhantomData<&'de ()>,
        }
        impl<'de> de::Visitor<'de> for Visitor<'de> {
            type Value = HeaderValue;
            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                Formatter::write_str(formatter, "tuple struct HeaderValue")
            }
            #[inline]
            fn visit_newtype_struct<E>(self, e: E) -> Result<Self::Value, E::Error>
            where
                E: Deserializer<'de>,
            {
                match <Vec<String> as Deserialize>::deserialize(e) {
                    Ok(val) => Ok(val.into()),
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            #[inline]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                match match de::SeqAccess::next_element::<Vec<String>>(&mut seq) {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(err);
                    }
                } {
                    Some(value) => Ok(value.into()),
                    None => {
                        return Err(de::Error::invalid_length(
                            0usize,
                            &"tuple struct HeaderValue with 1 element",
                        ));
                    }
                }
            }
        }
        Deserializer::deserialize_newtype_struct(
            deserializer,
            "HeaderValue",
            Visitor {
                marker: PhantomData::<HeaderValue>,
                lifetime: PhantomData,
            },
        )
    }
}

impl Serialize for HeaderValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let vec: Vec<String> = self
            .try_into()
            .map_err(|e| S::Error::custom(format!("Unknown character type: {}", e)))?;
        Serializer::serialize_newtype_struct(serializer, "HeaderValue", &vec)
    }
}

impl From<Vec<String>> for HeaderValue {
    fn from(t: Vec<String>) -> Self {
        let mut set = HashSet::new();
        for i in t {
            set.insert(i);
        }
        Self(set)
    }
}
impl From<Vec<&str>> for HeaderValue {
    fn from(t: Vec<&str>) -> Self {
        let mut set = HashSet::new();
        for i in t {
            set.insert(i.to_string());
        }
        Self(set)
    }
}

impl From<&str> for HeaderValue {
    fn from(s: &str) -> Self {
        let mut set = HashSet::new();
        set.insert(s.to_string());
        Self(set)
    }
}
impl From<String> for HeaderValue {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}
impl From<Option<()>> for HeaderValue {
    fn from(_: Option<()>) -> Self {
        Self::default()
    }
}

impl TryFrom<&HeaderValue> for Vec<String> {
    type Error = Utf8Error;

    fn try_from(t: &HeaderValue) -> Result<Self, Self::Error> {
        t.0.iter()
            .map(|t| from_utf8(t.as_ref()).map(|t| t.to_string()))
            .collect::<Result<Vec<String>, Utf8Error>>()
    }
}

impl From<http::HeaderValue> for HeaderValue {
    fn from(t: http::HeaderValue) -> Self {
        let mut values = HeaderValue::default();
        values.insert(String::from_utf8(Vec::from(t.as_bytes())).unwrap_or("".to_string()));
        values
    }
}

impl From<&http::HeaderValue> for HeaderValue {
    fn from(t: &http::HeaderValue) -> Self {
        let mut values = HeaderValue::default();
        values.insert(String::from_utf8(Vec::from(t.as_bytes())).unwrap_or("".to_string()));
        values
    }
}

impl Deref for HeaderValue {
    type Target = HashSet<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HeaderValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToString for HeaderValue {
    fn to_string(&self) -> String {
        self.0.iter().cloned().collect::<Vec<String>>().join("")
    }
}

impl TryFrom<&HeaderValue> for http::HeaderValue {
    type Error = String;
    fn try_from(t: &HeaderValue) -> Result<Self, Self::Error> {
        Self::from_bytes(t.as_vec_u8().as_slice()).map_err(|e| e.to_string())
    }
}
