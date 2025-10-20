pub mod request;
pub mod response;
pub mod route;
pub mod router;

pub use request::Request;
pub use response::Response;
pub use route::Route;
pub use router::Router;
use std::fmt::Display;

use std::collections::BTreeSet;
use std::path::PathBuf;

pub fn test_req() -> Request {
    Request {
        method: HttpMethod::Get,
        path: PathBuf::from("/").into_boxed_path(),
        version: HttpVersion::OnePointOne,
        headers: BTreeSet::new(),
        body: "".to_string(),
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    Get,
}
impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Get => "GET",
        };

        write!(f, "{}", out)
    }
}
impl HttpMethod {
    fn as_bytes(self) -> &'static [u8] {
        match self {
            Self::Get => b"GET",
        }
    }
}

#[derive(Debug)]
pub enum HttpVersion {
    OnePointOne,
}
impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::OnePointOne => "HTTP/1.1",
        };

        write!(f, "{}", out)
    }
}
impl HttpVersion {
    pub fn as_bytes(self) -> &'static [u8] {
        match self {
            Self::OnePointOne => b"HTTP/1.1",
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Header {
    title: String,
    val: String,
}
impl Header {
    pub fn as_bytes(&self) -> Vec<u8> {
        format!("{}: {}\r\n", self.title, self.val).into_bytes()
    }
}
