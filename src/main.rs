use tokio::{self, io::AsyncWriteExt, net::TcpListener};

use http_server::{
    HttpVersion, Request, Response, Router, response::StatusCode, route::HandlerError,
};

const ADDR: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
    let router = Router::builder()
        .get("/", get_index)
        .get("/users", get_users)
        .build()
        .unwrap();

    let listener = TcpListener::bind(ADDR)
        .await
        .expect("Cannot open socket on this port");

    println!("Listening on http://{}", ADDR);

    while let Ok((mut socket, _)) = listener.accept().await {
        let router_instance = router.clone();
        tokio::spawn(async move {
            match Request::try_from_socket(&mut socket).await {
                Ok(req) => {
                    let res = router_instance.route(req);
                    socket.write(res.as_bytes().as_slice()).await.unwrap();
                }
                Err(e) => println!("Uh oh: {:?}", e),
            }
        });
    }
}

fn get_index(_req: Request) -> Result<Response, HandlerError> {
    Ok(Response {
        version: HttpVersion::OnePointOne,
        code: StatusCode::Success,
        headers: Vec::new(),
        body: "<h1>HIIII</h1>".to_string(),
    })
}
fn get_users(_req: Request) -> Result<Response, HandlerError> {
    Ok(Response {
        version: HttpVersion::OnePointOne,
        code: StatusCode::Success,
        headers: Vec::new(),
        body: "<h1>OMG I DID IT</h1>".to_string(),
    })
}
