use std::error::Error;
use tokio::{
    io::copy_bidirectional,
    net::TcpStream,
};
use std::net::IpAddr;

// TODO: implement all the cool little different load balancing decision thingies

//TcpStream::connect(socket: String);

fn ipv4_to_u32(stream: &TcpStream) -> Result<u32, Box<dyn Error>> {
    match stream.peer_addr()?.ip() {
        IpAddr::V4(ipv4) => Ok(u32::from(ipv4)),
        _ => panic!("Expected IpAddr::V4"),
    }
}

async fn redirect(stream: &mut TcpStream, socket: &str) -> Result<(), Box<dyn Error>>{
    let mut backend = TcpStream::connect(socket).await?;
    copy_bidirectional(stream, &mut backend).await?;
    Ok(())
}

//statics
pub async fn ip_hash(mut stream: TcpStream, server_sockets: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let ip_int: usize = ipv4_to_u32(&stream)? as usize;
    let idx: usize = ip_int % server_sockets.len();
    let destination: &str = server_sockets[idx];
    println!("Redirecting to {}", destination);
    redirect(&mut stream, destination).await?;
    Ok(())
}

// pub fn round_robin() {}
// pub fn weighted_round_robin() {}

//dynamics
