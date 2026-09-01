use std::error::Error;
use tokio::{
    io::copy_bidirectional,
    net::TcpStream,
};
use std::net::IpAddr;

// TODO: implement all the cool little different load balancing decision thingies

//TcpStream::connect(socket: String);

// TODO: do the gcd thing lol
fn gcd() -> usize {}

fn ipv4_to_u32(stream: &TcpStream) -> Result<u32, Box<dyn Error>> {
    match stream.peer_addr()?.ip() {
        IpAddr::V4(ipv4) => Ok(u32::from(ipv4)),
        _ => panic!("Expected IpAddr::V4"),
    }
}

// TODO: use the LVS psuedocode to make the server sequence
//(https://kb.linuxvirtualserver.org/wiki/Weighted_Round-Robin_Scheduling)
fn make_sequence(server_sockets: &Vec<&str>, weights: &Vec<usize>, prev: usize) -> Result<Vec<&str>, Box<dyn Error>> {}

async fn redirect(stream: &mut TcpStream, socket: &str) -> Result<(), Box<dyn Error>> {
    let mut backend = TcpStream::connect(socket).await?;
    copy_bidirectional(stream, &mut backend).await?;
    Ok(())
}

//statics

//takes the previous index and returns the next one
pub async fn round_robin(mut stream: TcpStream, server_sockets: &Vec<&str>, prev: usize) -> Result<usize, Box<dyn Error>> {
    let next: usize = prev + 1;
    let idx = next % server_sockets.len();
    let destination: &str = server_sockets[idx];
    println!("Redirecting to {}", destination);
    redirect(&mut stream, destination).await?;
    Ok(next)
}

//round robin but with weights
pub async fn weighted_round_robin(mut stream: TcpStream, server_sockets: &Vec<&str>, weights: &Vec<usize>, prev: usize) -> Result<usize, Box<dyn Error>> {
    let destination: &str = server_sockets[idx];
    println!("Redirecting to {}", destination);
    redirect(&mut stream, destination).await?;
    Ok(next)
}

//integerizes the ip so the same ip connects to the same server
pub async fn ip_hash(mut stream: TcpStream, server_sockets: &Vec<&str>) -> Result<(), Box<dyn Error>> {
    let ip_int: usize = ipv4_to_u32(&stream)? as usize;
    let idx: usize = ip_int % server_sockets.len();
    let destination: &str = server_sockets[idx];
    println!("Redirecting to {}", destination);
    redirect(&mut stream, destination).await?;
    Ok(())
}

//dynamics
