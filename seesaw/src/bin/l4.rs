use std::error::Error;
use tokio::net::TcpListener;
use seesaw::techniques;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: make host and port read from config.toml
    // TODO: (maybe) make server_sockets read from config.toml & encrypt them so they arent in plaintext

    let listener_socket: &str = "127.0.0.1:8000";
    let server_sockets: Vec<&str> = vec![
        "127.0.0.1:8001",
        "127.0.0.1:8002",
        "127.0.0.1:8003",
    ];

    let listener = TcpListener::bind(listener_socket).await?;

    loop {
        let (stream, _addr) = listener.accept().await?;
        techniques::ip_hash(stream, &server_sockets).await?;
    }
}
