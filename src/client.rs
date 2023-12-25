mod core;
use crate::core::{Node};
use std::io::Write;
use std::net::{TcpStream, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::env;

fn register_node(addr: Ipv4Addr, port: u16) -> std::io::Result<()>{
    let socket_addr = SocketAddr::new(addr.into(), port);

    let mut stream = match TcpStream::connect(socket_addr) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    stream.write_all(b"Hello, server").unwrap();

    let node: Node<String, String> = Node::new(addr, port);
    
    Ok(())
}
fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        println!("usage: client <ip_addr>");
        std::process::exit(1);
    }

    let ip = match Ipv4Addr::from_str(&args[1]) {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Failed to parse IPv4 address from {} with error: {}", args[1], e);
            std::process::exit(1); 
        }
    };

    if let Err(e) = register_node(ip, 8080) {
        eprintln!("Failed to register node: {}", e);
    }
}