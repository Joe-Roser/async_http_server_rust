pub mod middleware;
pub mod request;
pub mod response;
pub mod route;
pub mod router;

pub use request::Request;
pub use response::Response;
pub use route::Route;
pub use router::Router;
use std::fmt::Display;

// The method of the request
#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Head = 1,     // Identical to Get, but without the body
    Get = 2,      // Gets specific data
    Post = 4,     // Submits an entity to the specified resource
    Put = 8,      // Replaces a current resource with the given resource
    Delete = 16,  // Deletes the specified resource
    Connect = 32, // Establishes a tunnel to the server specified
    Options = 64, // Describes the communication options for the specified resource
    Trace = 128,  // Still not sure tbh
    Patch = 256,  // Partial modification to a resource
}
impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Head => "HEAD",
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Patch => "PATCH",
        };

        write!(f, "{}", out)
    }
}
impl HttpMethod {
    pub fn display(&self) -> &'static str {
        match self {
            Self::Head => "HEAD",
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Patch => "PATCH",
        }
    }
    pub fn as_bytes(self) -> &'static [u8] {
        match self {
            Self::Head => b"HEAD",
            Self::Get => b"GET",
            Self::Post => b"POST",
            Self::Put => b"PUT",
            Self::Delete => b"DELETE",
            Self::Connect => b"CONNECT",
            Self::Options => b"OPTIONS",
            Self::Trace => b"TRACE",
            Self::Patch => b"PATCH",
        }
    }

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        match buf {
            b"HEAD" => Some(Self::Head),
            b"GET" => Some(Self::Get),
            b"POST" => Some(Self::Post),
            b"PUT" => Some(Self::Put),
            b"DELETE" => Some(Self::Delete),
            b"CONNECT" => Some(Self::Connect),
            b"OPTIONS" => Some(Self::Options),
            b"TRACE" => Some(Self::Trace),
            b"PATCH" => Some(Self::Patch),
            _ => None,
        }
    }
}

// The version of http you are using
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

// A single http header
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
