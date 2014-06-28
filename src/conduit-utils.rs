#![feature(macro_rules)]
#![feature(globs)]

extern crate semver;
extern crate conduit;

use std::io::net::ip::IpAddr;

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


#[cfg(test)]
mod tests {
    extern crate test = "conduit-test";
    use super::*;

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
}
