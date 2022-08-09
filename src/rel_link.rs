

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RelLink {
    pub(crate) href: String,
    pub(crate) rel: String,
    pub(crate) method: HttpMethod,
}

impl RelLink {
    pub fn new(href: &str, rel: &str, method: HttpMethod) -> Self {
        RelLink {
            href: href.to_string(),
            rel: rel.to_string(),
            method,
        }
    }
}

impl From<(String, String, HttpMethod)> for RelLink {
    fn from(r: (String, String, HttpMethod)) -> Self {
        Self::new(&r.0, &r.1, r.2)
    }
}

impl From<(&str, &str, HttpMethod)> for RelLink {
    fn from(r: (&str, &str, HttpMethod)) -> Self {
        Self::new(r.0, r.1, r.2)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}
