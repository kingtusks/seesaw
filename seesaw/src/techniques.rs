use std::error::Error;
use std::net::{
    TcpStream,
    IpAddr,
    SocketAddrV4,
};

// TODO: implement all the cool little different load balancing decision thingies

//TcpStream::connect(socket: String);

fn ipv4_to_u32(stream: &TcpStream) -> Result<u32, Box<dyn Error>> {
    match stream.peer_addr()?.ip() {
        IpAddr::V4(ipv4) => Ok(u32::from(ipv4)),
        _ => panic!("Expected IpAddr::V4"),
    }
}

//statics
pub fn ip_hash(stream: TcpStream, server_sockets: &Vec<SocketAddrV4>) -> Result<(), Box<dyn Error>> {
    let ip_int: usize = ipv4_to_u32(&stream)? as usize;
    let idx: usize = ip_int % server_sockets.len();
    let destination = server_sockets[idx];

    Ok(())
}

// pub fn round_robin() {}
// pub fn weighted_round_robin() {}

//dynamics
