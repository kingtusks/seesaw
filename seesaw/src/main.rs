use std::error::Error;
use std::net::{
    TcpListener,
    SocketAddrV4,
};
use seesaw::techniques;

fn main() -> Result<(), Box<dyn Error>> {
    //TODO: make host and port read from config.toml
    //TODO: (maybe) make server_sockets read from config.toml & encrypt them so they arent in plaintext
    let host = "127.0.0.1";
    let port = "8000";
    let server_sockets: Vec<SocketAddrV4> = vec![];

    let listener = TcpListener::bind(format!("{}:{}", host, port))?;

    for stream in listener.incoming() {
        techniques::round_robin(stream?, &server_sockets)?;
    }

    Ok(())
}
