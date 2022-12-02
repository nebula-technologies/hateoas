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
pub struct HeaderValue(HashSet<Bytes>);

impl HeaderValue {
    pub fn new_with_value<'a, B: Into<Vec<Bytes>>>(values: B) -> Self {
        let mut inner = HashSet::new();
        for v in values.into() {
            inner.insert(v);
        }
        Self(inner)
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
            .map_err(|e| S::Error::custom("Unknown character type"))?;
        Serializer::serialize_newtype_struct(serializer, "HeaderValue", &vec)
    }
}

impl From<Vec<String>> for HeaderValue {
    fn from(t: Vec<String>) -> Self {
        let mut set = HashSet::new();
        for i in t {
            set.insert(Bytes::from(i));
        }
        Self(set)
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

impl Deref for HeaderValue {
    type Target = HashSet<Bytes>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HeaderValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
