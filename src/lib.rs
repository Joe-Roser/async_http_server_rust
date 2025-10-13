use std::{collections::HashMap, fmt::Display, path::Path};
use tokio::io::AsyncReadExt;

const BUF_SIZE: usize = 2048;

struct RequestProccessor {
    state: RequestProccessorState,
    req_line: Option<RequestLine>,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}
enum RequestProccessorState {
    ReqLine,
    Headers,
    Body,
}

impl RequestProccessor {
    fn proccess_chunk(&mut self, buf: &[u8], n: usize) -> Result<(), RequestError> {
        for line in String::from_utf8_lossy(buf).lines() {
            match self.state {
                RequestProccessorState::ReqLine => {
                    let mut rl = line.split(" ");

                    let method = match rl.next() {
                        Some("GET") => RequestMethod::Get,
                        Some(a) => return Err(RequestError::MethodParseError(a.to_string())),
                        None => {
                            return Err(RequestError::MethodParseError(
                                "Not Enough Args".to_string(),
                            ));
                        }
                    };

                    let path = match rl.next() {
                        None => {
                            return Err(RequestError::MethodParseError(
                                "Not Enough Args".to_string(),
                            ));
                        }
                        Some(p) => p.to_string(),
                    };

                    let version = match rl.next() {
                        None => {
                            return Err(RequestError::MethodParseError(
                                "Not Enough Args".to_string(),
                            ));
                        }
                        Some(v) => v.to_string(),
                    };

                    if let Some(_) = rl.next() {
                        return Err(RequestError::MethodParseError("Too Many Args".to_string()));
                    };

                    self.req_line = Some(RequestLine {
                        method,
                        path,
                        version,
                    });

                    self.state = RequestProccessorState::Headers;
                }
                RequestProccessorState::Headers => todo!(),
                RequestProccessorState::Body => todo!(),
            }
        }
        Ok(())
    }

    fn finish(mut self) -> Result<Request, RequestError> {
        Err(RequestError::Unfinished)
    }
}

#[derive(Debug)]
pub struct Request {}
#[derive(Debug)]
pub struct RequestLine {
    method: RequestMethod,
    path: String,
    version: String,
}
#[derive(Debug)]
pub enum RequestMethod {
    Get,
}
pub enum RequestError {
    Unfinished,
    MethodParseError(String),
}

impl Request {
    fn new() -> RequestProccessor {
        RequestProccessor {
            state: RequestProccessorState::ReqLine,
            req_line: None,
            headers: None,
            body: None,
        }
    }
}
impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A Request")
    }
}

pub async fn parse_request<R>(mut stream: R) -> Result<Request, RequestError>
where
    R: AsyncReadExt + Unpin,
{
    let mut buf = [0u8; BUF_SIZE];
    let mut req = Request::new();
    loop {
        let n = stream.read(&mut buf).await.expect("Failed to read");
        println!("{}", str::from_utf8(&buf).unwrap());
        req.proccess_chunk(&buf, n)?;

        if n != BUF_SIZE {
            break;
        }
    }
    Ok(Request {})
}
