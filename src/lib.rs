#![feature(collections)]
#![cfg_attr(test, deny(warnings))]

extern crate semver;
extern crate conduit;

use std::iter;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::collections::hash_map::{HashMap, Iter};

use conduit::{Method, Scheme, Host, Extensions, Headers, Request};

pub trait RequestDelegator {
    fn request(&self) -> &Request;
    fn mut_request(&mut self) -> &mut Request;

    fn http_version(&self) -> semver::Version {
        self.request().http_version()
    }

    fn conduit_version(&self) -> semver::Version {
        self.request().conduit_version()
    }

    fn method(&self) -> Method {
        self.request().method()
    }

    fn scheme(&self) -> Scheme {
        self.request().scheme()
    }

    fn host(&self) -> Host {
        self.request().host()
    }

    fn virtual_root(&self) -> Option<&str> {
        self.request().virtual_root()
    }

    fn path(&self) -> &str {
        self.request().path()
    }

    fn query_string(&self) -> Option<&str> {
        self.request().query_string()
    }

    fn remote_addr(&self) -> SocketAddr {
        self.request().remote_addr()
    }

    fn content_length(&self) -> Option<u64> {
        self.request().content_length()
    }

    fn headers(&self) -> &Headers {
        self.request().headers()
    }

    fn body(&mut self) -> &mut Read {
        self.mut_request().body()
    }

    fn extensions(&self) -> &Extensions {
        self.request().extensions()
    }

    fn mut_extensions(&mut self) -> &mut Extensions {
        self.mut_request().mut_extensions()
    }
}

impl<'a> Request for &'a mut (RequestDelegator + 'a) {
    fn http_version(&self) -> semver::Version {
        (**self).http_version()
    }

    fn conduit_version(&self) -> semver::Version {
        (**self).conduit_version()
    }

    fn method(&self) -> Method {
        (**self).method()
    }

    fn scheme(&self) -> Scheme {
        (**self).scheme()
    }

    fn host(&self) -> Host {
        (**self).host()
    }

    fn virtual_root(&self) -> Option<&str> {
        (**self).virtual_root()
    }

    fn path(&self) -> &str {
        (**self).path()
    }

    fn query_string(&self) -> Option<&str> {
        (**self).query_string()
    }

    fn remote_addr(&self) -> SocketAddr {
        (**self).remote_addr()
    }

    fn content_length(&self) -> Option<u64> {
        (**self).content_length()
    }

    fn headers(&self) -> &Headers {
        (**self).headers()
    }

    fn body(&mut self) -> &mut Read {
        (**self).body()
    }

    fn extensions(&self) -> &Extensions {
        (**self).extensions()
    }

    fn mut_extensions(&mut self) -> &mut Extensions {
        (**self).mut_extensions()
    }
}

type RawHeaders = HashMap<String, Vec<String>>;
pub type InHeader<'a> = (&'a String, &'a Vec<String>);
pub type OutHeader<'a> = (String, &'a Vec<String>);

#[derive(PartialEq, Clone, Debug)]
pub struct HeaderMap(HashMap<String, Vec<String>>);

impl HeaderMap {
    pub fn normalize(headers: HashMap<String, Vec<String>>) -> HeaderMap {
        let headers = headers.into_iter().map(|(k,v)| (to_lower(&k), v)).collect();
        HeaderMap(headers)
    }

    pub fn iter(&self) -> iter::Map<Iter<String, Vec<String>>,
                                for<'a> fn(InHeader<'a>) -> OutHeader<'a>> {
        fn foo<'a>((k, v): InHeader<'a>) -> OutHeader<'a> { (to_lower(k), v) }
        let f: for<'a> fn(InHeader<'a>) -> OutHeader<'a> = foo;
        self.as_ref().iter().map(f)
    }

    fn as_ref(&self) -> &HashMap<String, Vec<String>> {
        match *self {
            HeaderMap(ref map) => map
        }
    }

    fn as_mut(&mut self) -> &mut HashMap<String, Vec<String>> {
        match *self {
            HeaderMap(ref mut map) => map
        }
    }

    pub fn len(&self) -> usize {
        self.as_ref().len()
    }
    pub fn clear(&mut self) {
        self.as_mut().clear()
    }
    pub fn find(&self, key: &str) -> Option<&Vec<String>> {
        self.as_ref().get(&to_lower(key))
    }
    pub fn insert(&mut self, k: &str, v: Vec<String>) -> Option<Vec<String>> {
        self.as_mut().insert(to_lower(&k), v)
    }
    pub fn remove(&mut self, k: &str) -> Option<Vec<String>> {
        self.as_mut().remove(&to_lower(k))
    }

    pub fn find_mut(&mut self, k: &str) -> Option<&mut Vec<String>> {
        self.as_mut().get_mut(&to_lower(k))
    }
}

fn to_lower(string: &str) -> String {
    string.to_lowercase()
}

#[cfg(test)]
mod tests {
    extern crate conduit_test as test;

    use {RequestDelegator, HeaderMap};

    use std::collections::HashMap;
    use conduit::{Request, Method};

    struct OverrideRequest<'a> {
        request: &'a mut (Request + 'a),
    }

    impl<'a> RequestDelegator for OverrideRequest<'a> {
        fn request(&self) -> &Request {
            let req: &Request = self.request; req
        }

        fn mut_request(&mut self) -> &mut Request {
            let req: &mut Request = self.request; req
        }

        fn method(&self) -> Method {
            Method::Get
        }
    }

    #[test]
    fn test_delegate() {
        let request = &mut test::MockRequest::new(Method::Head, "/hello");
        let new = OverrideRequest { request: request };

        assert_eq!(new.method(), Method::Get);
        assert_eq!(new.path(), "/hello");
    }

    #[test]
    fn test_header_map() {
        let mut map = HeaderMap(HashMap::new());
        map.insert("Content-Type", vec!("text/html".to_string()));
        map.insert("location", vec!("http://example.com".to_string()));

        assert_eq!(map.find(&"content-type".to_string()), Some(&vec!("text/html".to_string())));
        assert_eq!(map.find(&"Location".to_string()), Some(&vec!("http://example.com".to_string())));
        assert_eq!(map.find(&"content-type"), Some(&vec!("text/html".to_string())));
        assert_eq!(map.find(&"Location"), Some(&vec!("http://example.com".to_string())));
    }

    #[test]
    fn test_header_map_with_static_inserts() {
        let mut map = HeaderMap(HashMap::new());
        map.insert("Content-Type", vec!("text/html".to_string()));
        map.insert("location", vec!("http://example.com".to_string()));

        assert_eq!(map.find(&"content-type".to_string()), Some(&vec!("text/html".to_string())));
        assert_eq!(map.find(&"Location".to_string()), Some(&vec!("http://example.com".to_string())));
        assert_eq!(map.find(&"content-type"), Some(&vec!("text/html".to_string())));
        assert_eq!(map.find(&"Location"), Some(&vec!("http://example.com".to_string())));
    }

    #[test]
    fn test_normalize() {
        let mut map = HashMap::new();
        map.insert("Content-Type".to_string(), vec!("text/html".to_string()));

        let headers = HeaderMap::normalize(map);
        assert_eq!(headers.find(&"Content-Type".to_string()), Some(&vec!("text/html".to_string())));
        assert_eq!(headers.find(&"Content-Type"), Some(&vec!("text/html".to_string())));
        assert_eq!(headers.find(&"content-type".to_string()), Some(&vec!("text/html".to_string())));
        assert_eq!(headers.find(&"content-type"), Some(&vec!("text/html".to_string())));
    }

    #[test]
    fn test_iterate() {
        let mut headers = HeaderMap(HashMap::new());
        headers.insert("Content-Type", vec!("text/html".to_string()));
        headers.insert("location", vec!("http://example.com".to_string()));

        assert!(headers.iter().any(|t| {
            t.0 == "content-type" &&
            &t.1[..] == ["text/html".to_string()]
        }));
        assert!(headers.iter().any(|t| {
            t.0 == "location" &&
            &t.1[..] == ["http://example.com".to_string()]
        }));
        assert!(headers.iter().count() == 2);
    }
}
