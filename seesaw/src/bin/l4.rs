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
    let weights: Vec<usize> = vec![1, 2, 3]; //for techniques::weighted_round_robin

    let listener = TcpListener::bind(listener_socket).await?;
    let mut next = 0; //for techniques::round_robin

    loop {
        let (stream, _) = listener.accept().await?;
        next = techniques::round_robin(stream, &server_sockets, next).await?;
        // techniques::ip_hash(stream?, &server_sockets).await?;
    }
}
