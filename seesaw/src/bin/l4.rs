use std::error::Error;
use std::net::{
    TcpListener,
    SocketAddrV4,
    Ipv4Addr,
};
use seesaw::techniques;

fn main() -> Result<(), Box<dyn Error>> {
    //TODO: make host and port read from config.toml
    //TODO: (maybe) make server_sockets read from config.toml & encrypt them so they arent in plaintext

    let listener_socket = "127.0.0.1:8000";
    let localhost_ipv4 = Ipv4Addr::new(127, 0, 0, 1);
    let server_sockets: Vec<SocketAddrV4> = vec![
        SocketAddrV4::new(localhost_ipv4, 8080),
        SocketAddrV4::new(localhost_ipv4, 8081),
        SocketAddrV4::new(localhost_ipv4, 8082),
    ];

    let listener = TcpListener::bind(listener_socket)?;

    for stream in listener.incoming() {
        techniques::ip_hash(stream?, &server_sockets)?;
    }

    Ok(())
}
