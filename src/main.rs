use tokio::{self, net::TcpListener};

use http_server::parse_request;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Cannot open socket on this port");

    println!("Listening on http://{}", addr);

    while let Ok((mut socket, _)) = listener.accept().await {
        tokio::spawn(async move {
            match parse_request(&mut socket).await {
                Ok(req) => println!("{}", req),
                _ => {}
            }
        });
    }
}
