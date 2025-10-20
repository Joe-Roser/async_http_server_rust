use tokio::{self, io::AsyncWriteExt, net::TcpListener};

use http_server::{
    Request, Response, Router, request::RequestMethod, response::ResponseCode, route::HandlerError,
};

#[tokio::main]
async fn main() {
    let router = Router::builder().get("/", get_index).build().unwrap();

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Cannot open socket on this port");

    println!("Listening on http://{}", addr);

    while let Ok((mut socket, _)) = listener.accept().await {
        println!("connected");
        let router_instance = router.clone();
        tokio::spawn(async move {
            match Request::try_from_socket(&mut socket).await {
                Ok(req) => {
                    let res = router_instance.route(req);
                    socket.write(res.to_bytes()).await.unwrap();
                    println!("Served!!");
                }
                Err(e) => println!("Uh oh: {:?}", e),
            }
        });
    }
}

fn get_index(_req: Request) -> Result<Response, HandlerError> {
    Ok(Response {
        method: RequestMethod::Get,
        code: ResponseCode::Success,
        headers: Vec::new(),
        body: "<h1>HIIII</h1>".to_string(),
    })
}
