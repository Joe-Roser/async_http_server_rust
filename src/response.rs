use std::fmt::Display;

use crate::request::{Header, RequestMethod};

#[derive(Debug)]
pub struct Response {
    pub method: RequestMethod,
    pub code: ResponseCode,
    pub headers: Vec<Header>,
    pub body: String,
}

impl Response {
    pub fn not_found() -> Response {
        Response {
            method: RequestMethod::Get,
            code: ResponseCode::ServerError,
            headers: Vec::new(),
            body: String::new(),
        }
    }

    pub fn to_bytes(self) -> Box<[u8]> {}
}

#[derive(Debug)]
pub enum ResponseCode {
    Success,
    ServerError,
}

impl Display for ResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseCode::Success => write!(f, "200 SUCCESS"),
            ResponseCode::ServerError => write!(f, "400 INTERNAL SERVER ERROR"),
        }
    }
}
