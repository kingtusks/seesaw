use std::error::Error;
use std::net::{
    TcpListener,
    TcpStream,
    IpAddr,
    // Ipv4Addr,
    SocketAddrV4,
};

//TODO: implement all the cool little different load balancing decision thingies

//TcpStream::connect(socket: String);

fn redirect(stream: TcpStream, server_sockets: &Vec<SocketAddrV4>) -> Result<(), Box<dyn Error>> {
    let ip: IpAddr = stream.peer_addr()?.ip();
    let n = server_sockets.len();
    // let ip_int: u32 = u32::from(ip);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    //TODO: make host and port read from config.toml
    let host = "127.0.0.1";
    let port = "8000";

    let listener = TcpListener::bind(format!("{}:{}", host, port))?;
    //TODO: (maybe) make server_sockets read from config.toml & encrypt them so they arent in plaintext
    let server_sockets: Vec<SocketAddrV4> = vec![];

    for stream in listener.incoming() {
        let _ = redirect(stream?, &server_sockets);
    }

    Ok(())
}
