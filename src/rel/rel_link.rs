use crate::header::HeaderMap;
use crate::http_method::HttpMethod;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelLink {
    pub(crate) href: String,
    pub(crate) rel: String,
    pub(crate) method: HttpMethod,
    pub(crate) headers: HeaderMap,
}

impl RelLink {
    /// Constructs a new `RelLink`
    ///
    /// ```
    /// use hateoas::{HttpMethod, RelLink};
    ///
    /// let rel = RelLink::new( "somewhere_obj","/somewhere/", HttpMethod::Get, None);
    /// let validate = ("somewhere_obj", "/somewhere/", HttpMethod::Get, None).into();
    ///
    /// assert_eq!(rel, validate);
    /// ```
    pub fn new<H: Into<HeaderMap>>(rel: &str, href: &str, method: HttpMethod, headers: H) -> Self {
        RelLink {
            href: href.to_string(),
            rel: rel.to_string(),
            method,
            headers: headers.into(),
        }
    }

    /// ## Getter for href
    ///
    /// ```
    /// use hateoas::{HttpMethod, RelLink};
    ///
    /// let rel = RelLink::new( "somewhere_obj","/somewhere/", HttpMethod::Get, None);
    ///
    /// assert_eq!(rel.href(), "/somewhere/");
    /// ```
    pub fn href(&self) -> &str {
        &self.href
    }

    /// ## Getter/Setter for href
    ///
    /// ```
    /// use hateoas::{HttpMethod, RelLink};
    ///
    /// let mut rel = RelLink::new( "somewhere_obj","/somewhere/", HttpMethod::Get, None);
    ///
    /// *(rel.href_mut()) = "/somewhere_else/".to_string();
    ///
    /// assert_eq!(rel.href(), "/somewhere_else/");
    /// ```
    pub fn href_mut(&mut self) -> &mut String {
        &mut self.href
    }

    /// ## Getter for rel
    ///
    /// ```
    /// use hateoas::{HttpMethod, RelLink};
    ///
    /// let mut rel = RelLink::new( "somewhere_obj","/somewhere/", HttpMethod::Get, None);
    ///    ///
    /// assert_eq!(rel.rel(), "somewhere_obj");
    /// ```
    pub fn rel(&self) -> &str {
        &self.rel
    }

    /// ## Getter/Setter for rel
    ///
    /// ```
    /// use hateoas::{HttpMethod, RelLink};
    ///
    /// let mut rel = RelLink::new( "somewhere_obj","/somewhere/", HttpMethod::Get, None);
    ///
    /// *(rel.rel_mut()) =  "somewhere_obj_2".to_string();
    ///
    /// assert_eq!(rel.rel(),  "somewhere_obj_2");
    /// ```
    pub fn rel_mut(&mut self) -> &mut String {
        &mut self.rel
    }

    /// ## Getter for method
    ///
    /// ```
    /// use hateoas::{HttpMethod, RelLink};
    ///
    /// let mut rel = RelLink::new( "somewhere_obj","/somewhere/", HttpMethod::Get, None);    ///
    ///
    /// assert_eq!(rel.method(), &HttpMethod::Get);
    /// ```
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    /// ## Getter/Setter for rel
    ///
    /// ```
    /// use hateoas::{HttpMethod, RelLink};
    ///
    /// let mut rel = RelLink::new( "somewhere_obj","/somewhere/", HttpMethod::Get, None);
    ///
    /// *(rel.method_mut()) = HttpMethod::Connect;
    ///
    /// assert_eq!(rel.method(),  &HttpMethod::Connect);
    /// ```
    pub fn method_mut(&mut self) -> &mut HttpMethod {
        &mut self.method
    }

    /// ## Getter for headers
    ///
    /// ```
    /// use hateoas::{HttpMethod, RelLink};
    ///
    /// let mut rel = RelLink::new( "somewhere_obj","/somewhere/", HttpMethod::Get, ("Content-Type", "application/json"));    ///
    ///
    /// assert_eq!(rel.method(), &HttpMethod::Get);
    /// ```
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// ## Getter/Setter for headers
    ///
    /// ```
    /// use hateoas::{HttpMethod, RelLink};
    ///
    /// let mut rel = RelLink::new( "somewhere_obj","/somewhere/", HttpMethod::Get, None);
    ///
    /// rel.headers_mut().append("Content-Type", "application/json");
    ///
    /// assert!(rel.headers().contains_key("Content-Type"));
    /// ```
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }
}

impl From<(String, String, HttpMethod)> for RelLink {
    fn from(r: (String, String, HttpMethod)) -> Self {
        Self::new(&r.0, &r.1, r.2, None)
    }
}

impl From<(&str, &str, HttpMethod)> for RelLink {
    fn from(r: (&str, &str, HttpMethod)) -> Self {
        Self::new(r.0, r.1, r.2, None)
    }
}

impl From<(&str, &str, HttpMethod, (&str, &str))> for RelLink {
    fn from(r: (&str, &str, HttpMethod, (&str, &str))) -> Self {
        Self::new(r.0, r.1, r.2, HeaderMap::from(r.3))
    }
}

impl From<(&str, &str, HttpMethod, Vec<(&str, &str)>)> for RelLink {
    fn from(r: (&str, &str, HttpMethod, Vec<(&str, &str)>)) -> Self {
        Self::new(r.0, r.1, r.2, HeaderMap::from(r.3))
    }
}

impl From<(&str, &str, HttpMethod, HeaderMap)> for RelLink {
    fn from(r: (&str, &str, HttpMethod, HeaderMap)) -> Self {
        Self::new(r.0, r.1, r.2, HeaderMap::from(r.3))
    }
}

impl From<(&str, &str, HttpMethod, Option<()>)> for RelLink {
    fn from(r: (&str, &str, HttpMethod, Option<()>)) -> Self {
        Self::new(r.0, r.1, r.2, HeaderMap::from(r.3))
    }
}

macro_rules! relational_links {
    (
        $(
            $(#[$docs:meta])*
            ($konst:ident, $function:ident);
        )+
    ) => {
        impl RelLink {
        $(
            $(#[$docs])*
            #[doc = " ```\n" ]
            #[doc = " use hateoas::{HttpMethod, RelLink};\n"]
            #[doc = " \n" ]
            #[doc = concat!(" let rel = RelLink::", stringify!($function), "(\"object\", \"/path/to/objects\");\n") ]
            #[doc = " \n" ]
            #[doc = concat!(" assert_eq!(rel, RelLink::new(\"object\", \"/path/to/objects\", HttpMethod::", stringify!($konst), ", None));\n") ]
            #[doc = " ``` "]
            #[allow(non_snake_case)]
            pub fn $function(rel: &str, href: &str) -> RelLink {
                RelLink{
                    href: format!("{}",href),
                    rel: format!("{}",rel),
                    method: HttpMethod::$konst,
                    headers: HeaderMap::default()
                }
            }
        )+
        }
    }
}

relational_links! {
    /// The GET method requests a representation of the specified resource. Requests using GET should only retrieve data.
    (Get, GET);

    /// The HEAD method asks for a response identical to a GET request, but without the response body.
    (Head, HEAD);

    /// The POST method submits an entity to the specified resource, often causing a change in state or side effects on the server.
    (Post, POST);

    /// The PUT method replaces all current representations of the target resource with the request payload.
    (Put, PUT);

    /// The DELETE method deletes the specified resource.
    (Delete, DELETE);

    /// The CONNECT method establishes a tunnel to the server identified by the target resource.
    (Connect, CONNECT);

    /// The OPTIONS method describes the communication options for the target resource.
    (Options, OPTIONS);

    /// The TRACE method performs a message loop-back test along the path to the target resource.
    (Trace, TRACE);

    /// The PATCH method applies partial modifications to a resource.
    (Patch, PATCH);
}
