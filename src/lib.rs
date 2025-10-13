use std::fmt::Display;
use tokio::io::AsyncReadExt;

const BUF_SIZE: usize = 2048;

pub async fn parse_request<R>(mut stream: R) -> Result<Request, RequestError>
where
    R: AsyncReadExt + Unpin,
{
    let mut buf = [0u8; BUF_SIZE];
    loop {
        let n = stream.read(&mut buf).await.expect("Failed to read");
        println!("{}", str::from_utf8(&buf).unwrap());

        if n != BUF_SIZE {
            break;
        }
    }
    Ok(Request {})
}

#[derive(Debug)]
pub struct Request {}
pub enum RequestError {}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A Request")
    }
}
