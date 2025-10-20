use std::fmt::Display;

use crate::Header;
use crate::HttpMethod;
use crate::HttpVersion;

#[derive(Debug)]
pub struct Response {
    pub version: HttpVersion,
    pub code: StatusCode,
    pub headers: Vec<Header>,
    pub body: String,
}

impl Response {
    pub fn not_found() -> Response {
        Response {
            version: HttpVersion::OnePointOne,
            code: StatusCode::ServerError,
            headers: Vec::new(),
            body: String::new(),
        }
    }

    pub fn as_bytes(self) -> Vec<u8> {
        let mut resp = Vec::with_capacity(2048);

        resp.extend_from_slice(self.version.as_bytes());
        resp.push(b' ');
        resp.extend_from_slice(self.code.as_bytes());
        resp.extend_from_slice(b"\r\n");

        let mut content_length = false;
        for header in self.headers.iter().to_owned() {
            if &header.title == "content-length" {
                content_length = true;
            }
            resp.extend_from_slice(header.as_bytes().as_slice());
        }
        if !content_length {
            resp.extend_from_slice(b"content-length: ");
            resp.extend_from_slice(self.body.len().to_string().as_bytes());
            resp.extend_from_slice(b"\r\n");
        }

        resp.extend_from_slice(b"\r\n");
        resp.extend_from_slice(self.body.as_bytes());

        resp
    }
}

#[derive(Debug)]
pub enum StatusCode {
    Success,
    ServerError,
}
impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::Success => write!(f, "200 SUCCESS"),
            StatusCode::ServerError => write!(f, "400 INTERNAL SERVER ERROR"),
        }
    }
}
impl StatusCode {
    pub fn as_bytes(self) -> &'static [u8] {
        match self {
            Self::Success => b"200 SUCCESS",
            Self::ServerError => b"400 INTERNAL SERVER ERROR",
        }
    }
}
