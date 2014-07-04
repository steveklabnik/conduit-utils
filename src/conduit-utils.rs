#![feature(macro_rules)]
#![feature(globs)]

extern crate semver;
extern crate conduit;

use std::iter;
use std::io::net::ip::IpAddr;
use std::collections::hashmap::{HashMap, Entries};

use conduit::{Method, Scheme, Host, Extensions, Headers, Request};

pub trait RequestDelegator {
    fn request<'a>(&'a self) -> &'a Request;
    fn mut_request<'a>(&'a mut self) -> &'a mut Request;

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

    fn host<'a>(&'a self) -> Host<'a> {
        self.request().host()
    }

    fn virtual_root<'a>(&'a self) -> Option<&'a str> {
        self.request().virtual_root()
    }

    fn path<'a>(&'a self) -> &'a str {
        self.request().path()
    }

    fn query_string<'a>(&'a self) -> Option<&'a str> {
        self.request().query_string()
    }

    fn remote_ip(&self) -> IpAddr {
        self.request().remote_ip()
    }

    fn content_length(&self) -> Option<uint> {
        self.request().content_length()
    }

    fn headers<'a>(&'a self) -> &'a Headers {
        self.request().headers()
    }

    fn body<'a>(&'a mut self) -> &'a mut Reader {
        self.mut_request().body()
    }

    fn extensions<'a>(&'a self) -> &'a Extensions {
        self.request().extensions()
    }

    fn mut_extensions<'a>(&'a mut self) -> &'a mut Extensions {
        self.mut_request().mut_extensions()
    }
}

impl<'a> Request for &'a mut RequestDelegator {
    fn http_version(&self) -> semver::Version {
        self.http_version()
    }

    fn conduit_version(&self) -> semver::Version {
        self.conduit_version()
    }

    fn method(&self) -> Method {
        self.method()
    }

    fn scheme(&self) -> Scheme {
        self.scheme()
    }

    fn host<'a>(&'a self) -> Host<'a> {
        self.host()
    }

    fn virtual_root<'a>(&'a self) -> Option<&'a str> {
        self.virtual_root()
    }

    fn path<'a>(&'a self) -> &'a str {
        self.path()
    }

    fn query_string<'a>(&'a self) -> Option<&'a str> {
        self.query_string()
    }

    fn remote_ip(&self) -> IpAddr {
        self.remote_ip()
    }

    fn content_length(&self) -> Option<uint> {
        self.content_length()
    }

    fn headers<'a>(&'a self) -> &'a Headers {
        self.headers()
    }

    fn body<'a>(&'a mut self) -> &'a mut Reader {
        self.body()
    }

    fn extensions<'a>(&'a self) -> &'a Extensions {
        self.extensions()
    }

    fn mut_extensions<'a>(&'a mut self) -> &'a mut Extensions {
        self.mut_extensions()
    }
}

type RawHeaders = HashMap<String, Vec<String>>;
type InHeader<'a> = (&'a String, &'a Vec<String>);
type OutHeader<'a> = (String, &'a Vec<String>);

#[deriving(PartialEq, Clone, Show)]
pub struct HeaderMap(HashMap<String, Vec<String>>);

impl HeaderMap {
    fn normalize(headers: HashMap<String, Vec<String>>) -> HeaderMap {
        let headers = headers.move_iter().map(|(k,v)| (to_lower(&k), v)).collect();
        HeaderMap(headers)
    }

    fn iter<'a>(&'a self) -> iter::Map<'a, InHeader<'a>, OutHeader<'a>, Entries<'a, String, Vec<String>>> {
        self.as_ref().iter().map(|(k,v)| (to_lower(k), v))
    }

    fn as_ref<'a>(&'a self) -> &'a HashMap<String, Vec<String>> {
        match *self {
            HeaderMap(ref map) => map
        }
    }

    fn as_mut<'a>(&'a mut self) -> &'a mut HashMap<String, Vec<String>> {
        match *self {
            HeaderMap(ref mut map) => map
        }
    }
}

impl Collection for HeaderMap {
    fn len(&self) -> uint {
        self.as_ref().len()
    }
}

impl Mutable for HeaderMap {
    fn clear(&mut self) {
        self.as_mut().clear()
    }
}

impl<S: Str> Map<S, Vec<String>> for HeaderMap {
    fn find<'a>(&'a self, key: &S) -> Option<&'a Vec<String>> {
        self.as_ref().find(&to_lower(key))
    }
}

impl<S: Str> MutableMap<S, Vec<String>> for HeaderMap {
    fn swap(&mut self, k: S, v: Vec<String>) -> Option<Vec<String>> {
        self.as_mut().swap(to_lower(&k), v)
    }

    fn pop(&mut self, k: &S) -> Option<Vec<String>> {
        self.as_mut().pop(&to_lower(k))
    }

    fn find_mut<'a>(&'a mut self, k: &S) -> Option<&'a mut Vec<String>> {
        self.as_mut().find_mut(&to_lower(k))
    }
}

fn to_lower<S: Str>(string: &S) -> String {
    string.as_slice().chars().map(|c| c.to_lowercase()).collect()
}

#[cfg(test)]
mod tests {
    extern crate test = "conduit-test";
    use super::*;

    use std::collections::HashMap;
    use conduit;
    use conduit::{Request, Method};

    struct OverrideRequest<'a> {
        request: &'a mut Request
    }

    impl<'a> RequestDelegator for OverrideRequest<'a> {
        fn request<'a>(&'a self) -> &'a Request {
            let req: &Request = self.request; req
        }

        fn mut_request<'a>(&'a mut self) -> &'a mut Request {
            let req: &mut Request = self.request; req
        }

        fn method(&self) -> Method {
            conduit::Get
        }
    }

    #[test]
    fn test_delegate() {
        let request = &mut test::MockRequest::new(conduit::Head, "/hello") as &mut Request;
        let override = OverrideRequest { request: request };

        assert_eq!(override.method(), conduit::Get);
        assert_eq!(override.path(), "/hello");
    }

    #[test]
    fn test_header_map() {
        let mut map = HeaderMap(HashMap::new());
        map.insert("Content-Type".to_str(), vec!("text/html".to_str()));
        map.insert("location".to_str(), vec!("http://example.com".to_str()));

        assert_eq!(map.find(&"content-type".to_str()), Some(&vec!("text/html".to_str())))
        assert_eq!(map.find(&"Location".to_str()), Some(&vec!("http://example.com".to_str())))
        assert_eq!(map.find(&"content-type"), Some(&vec!("text/html".to_str())))
        assert_eq!(map.find(&"Location"), Some(&vec!("http://example.com".to_str())))
    }

    #[test]
    fn test_header_map_with_static_inserts() {
        let mut map = HeaderMap(HashMap::new());
        map.insert("Content-Type", vec!("text/html".to_str()));
        map.insert("location", vec!("http://example.com".to_str()));

        assert_eq!(map.find(&"content-type".to_str()), Some(&vec!("text/html".to_str())))
        assert_eq!(map.find(&"Location".to_str()), Some(&vec!("http://example.com".to_str())))
        assert_eq!(map.find(&"content-type"), Some(&vec!("text/html".to_str())))
        assert_eq!(map.find(&"Location"), Some(&vec!("http://example.com".to_str())))
    }

    #[test]
    fn test_normalize() {
        let mut map = HashMap::new();
        map.insert("Content-Type".to_str(), vec!("text/html".to_str()));

        let headers = HeaderMap::normalize(map);
        assert_eq!(headers.find(&"Content-Type".to_str()), Some(&vec!("text/html".to_str())))
        assert_eq!(headers.find(&"Content-Type"), Some(&vec!("text/html".to_str())))
        assert_eq!(headers.find(&"content-type".to_str()), Some(&vec!("text/html".to_str())))
        assert_eq!(headers.find(&"content-type"), Some(&vec!("text/html".to_str())))
    }

    #[test]
    fn test_iterate() {
        let mut headers = HeaderMap(HashMap::new());
        headers.insert("Content-Type", vec!("text/html".to_str()));
        headers.insert("location", vec!("http://example.com".to_str()));

        assert!(headers.iter().any(|t| t == ("content-type".to_str(), &vec!("text/html".to_str()))));
        assert!(headers.iter().any(|t| t == ("location".to_str(), &vec!("http://example.com".to_str()))));
        assert!(headers.iter().count() == 2);
    }
}
