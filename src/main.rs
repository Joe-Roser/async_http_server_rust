use tokio::{self, net::TcpListener};

use http_server::Request;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Cannot open socket on this port");

    println!("Listening on http://{}", addr);

    while let Ok((mut socket, _)) = listener.accept().await {
        println!("connected");
        tokio::spawn(async move {
            match Request::try_from_socket(&mut socket).await {
                Ok(req) => println!("{}", req),
                Err(e) => println!("Uh oh: {:?}", e),
            }
        });
    }
}
