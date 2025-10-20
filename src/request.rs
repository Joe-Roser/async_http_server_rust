use std::{
    collections::BTreeSet,
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use tokio::io::AsyncReadExt;

// The Request struct
//
// Stores all the imporant data from a request in a more usable formmat
#[derive(Debug)]
pub struct Request {
    pub(crate) method: RequestMethod,
    pub(crate) path: Box<Path>,
    pub(crate) version: HttpVersion,
    pub(crate) headers: BTreeSet<Header>,
    pub(crate) body: String,
}

impl Request {
    // Takes in a socket and returns a result on wether it could parse a http request from it.
    pub async fn try_from_socket<S: AsyncReadExt + Unpin>(
        socket: S,
    ) -> Result<Request, RequestError> {
        RequestParser::new().parse_from_socket(socket).await
    }

    pub fn get_path<'a>(&'a self) -> Box<Path> {
        self.path.clone()
    }
}
impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}\n",
            self.method,
            self.path.to_str().unwrap(),
            self.version
        )?;

        for h in self.headers.iter().clone() {
            write!(f, "{}: {}\n", h.title, h.val)?;
        }

        write!(f, "\n\n{}", self.body)
    }
}

#[derive(Debug)]
pub enum RequestMethod {
    Get,
}

impl Display for RequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Get => "GET",
        };

        write!(f, "{}", out)
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Header {
    title: String,
    val: String,
}

#[derive(Debug)]
pub enum RequestError {
    Unfinished,
    SocketReadError,
    InvalidUtf8,
    BadPath,
    BadVersion,
    BadHeader,
    RlParseError(String),
}

const BUF_SIZE: usize = 2048;

// The struct that does the actual parsing
//
#[derive(Debug)]
struct RequestParser {
    state: RequestParserState,
    method: Option<RequestMethod>,
    path: Option<PathBuf>,
    version: Option<HttpVersion>,
    headers: BTreeSet<Header>,
    body: String,
}

#[derive(Debug, PartialEq)]
enum RequestParserState {
    ReqLine,
    Headers,
    Body,
}

impl RequestParser {
    fn new() -> RequestParser {
        RequestParser {
            state: RequestParserState::ReqLine,
            method: None,
            path: None,
            version: None,
            headers: BTreeSet::new(),
            body: String::new(),
        }
    }
    // The actual meat of the parsing
    async fn parse_from_socket<S: AsyncReadExt + Unpin>(
        mut self,
        mut socket: S,
    ) -> Result<Request, RequestError> {
        let mut buf = [0u8; BUF_SIZE];
        let mut hang = String::new();

        loop {
            // Read a new chunk
            let n = socket
                .read(&mut buf)
                .await
                .map_err(|_| RequestError::SocketReadError)?;

            println!("{}", String::from_utf8_lossy(&buf));

            // Make sure chunk ends in a complete line
            let chunk = str::from_utf8(&buf[..n]).map_err(|_| RequestError::InvalidUtf8)?;
            hang.push_str(chunk);

            while let Some(idx) = hang.find('\r') {
                hang = self.parse_chunk(hang, idx)?;
            }

            if n != BUF_SIZE {
                break;
            }
        }
        Ok(self.to_request()?)
    }

    // Takes in a line and deals with it
    fn parse_chunk(&mut self, hang: String, idx: usize) -> Result<String, RequestError> {
        let line = &hang[..idx];

        match self.state {
            RequestParserState::ReqLine => {
                let mut rl = line.split(" ");

                match rl.next() {
                    Some("GET") => self.method = Some(RequestMethod::Get),
                    Some(_) => {
                        return Err(RequestError::RlParseError("Not a valid method".to_string()));
                    }
                    None => {
                        return Err(RequestError::RlParseError(
                            "No method on request".to_string(),
                        ));
                    }
                }
                match rl.next() {
                    Some(p) => {
                        self.path = Some(PathBuf::from_str(p).map_err(|_| RequestError::BadPath)?)
                    }
                    None => return Err(RequestError::BadPath),
                }
                match rl.next() {
                    Some("HTTP/1.1") => self.version = Some(HttpVersion::OnePointOne),
                    Some(_) => return Err(RequestError::BadVersion),
                    None => return Err(RequestError::BadVersion),
                }

                self.state = RequestParserState::Headers;
            }
            RequestParserState::Headers => {
                if line == String::new() {
                    self.state = RequestParserState::Body;
                    println!("headers done");
                } else {
                    let (title, val) = line.rsplit_once(": ").ok_or(RequestError::BadHeader)?;
                    self.headers.insert(Header {
                        title: title.to_string(),
                        val: val.to_string(),
                    });
                }
            }
            RequestParserState::Body => {
                println!("{:?}", self);
                println!("Got to body!!!");
                self.body.push_str(line);
            }
        }

        Ok(hang[idx + 2..].to_string())
    }

    // Converts the builder to an actual request
    fn to_request(self) -> Result<Request, RequestError> {
        let method = self.method.ok_or(RequestError::Unfinished)?;
        let path = self.path.ok_or(RequestError::Unfinished)?.into_boxed_path();
        let version = self.version.ok_or(RequestError::Unfinished)?;
        let headers = self.headers;
        let body = self.body;

        Ok(Request {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}
