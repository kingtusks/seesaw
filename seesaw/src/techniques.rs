use std::error::Error;
use tokio::{
    io::copy_bidirectional,
    net::TcpStream,
};
use std::net::IpAddr;

// TODO: implement all the cool little different load balancing decision thingies

//TcpStream::connect(socket: String);

// TODO:
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {a} else {gcd(b, a % b)}
}

fn ipv4_to_u32(stream: &TcpStream) -> Result<u32, Box<dyn Error>> {
    match stream.peer_addr()?.ip() {
        IpAddr::V4(ipv4) => Ok(u32::from(ipv4)),
        _ => panic!("Expected IpAddr::V4"),
    }
}

//makes a sequence with weights and outputs a Vec of indices
pub fn make_sequence(weights: Vec<usize>) -> Vec<usize> {
    let n = weights.len();
    let s = weights.iter()
        .sum();
    let weight_gcd = weights.iter()
        .copied()
        .fold(0, gcd);
    let weight_max = *weights.iter()
        .max()
        .unwrap_or(&0);

    let mut i: isize = -1;
    let mut cw: isize = 0;
    let mut sequence = Vec::with_capacity(s);

    for _ in 0..s {
        loop {
            i = (i + 1) % n as isize;
            if i == 0 {
                cw -= weight_gcd as isize;
                if cw <= 0 {
                    cw = weight_max as isize;
                    if cw == 0 {
                        return sequence;
                    }
                }
            }
            if weights[i as usize] as isize >= cw {
                sequence.push(i as usize);
                break;
            }
        }
    }
    sequence
}

async fn redirect(stream: &mut TcpStream, socket: &str) -> Result<(), Box<dyn Error>> {
    let mut backend = TcpStream::connect(socket).await?;
    copy_bidirectional(stream, &mut backend).await?;
    Ok(())
}

//statics

//takes the previous index and returns the next one
pub async fn round_robin(mut stream: TcpStream, server_sockets: &Vec<&str>, prev: usize) -> Result<usize, Box<dyn Error>> {
    let next: usize = prev + 1;
    let idx: usize = next % server_sockets.len();
    let destination: &str = server_sockets[idx];
    println!("Redirecting to {}", destination);
    redirect(&mut stream, destination).await?;
    Ok(next)
}

//round robin but with weights
pub async fn weighted_round_robin(mut stream: TcpStream, server_sockets: &Vec<&str>, sequence: &Vec<usize>, prev: usize) -> Result<usize, Box<dyn Error>> {
    let next: usize = prev + 1;
    let idx_for_sequence: usize = next % sequence.len();
    let destination: &str = server_sockets[sequence[idx_for_sequence]];
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
