pub mod request;
pub mod response;
pub mod route;
pub mod router;

pub use request::Request;
pub use response::Response;
pub use route::Route;
pub use router::Router;

use request::RequestMethod;
use std::collections::BTreeSet;
use std::path::PathBuf;

pub fn test_req() -> Request {
    Request {
        method: RequestMethod::Get,
        path: PathBuf::from("/").into_boxed_path(),
        version: request::HttpVersion::OnePointOne,
        headers: BTreeSet::new(),
        body: "".to_string(),
    }
}
