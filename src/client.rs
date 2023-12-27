mod core;
mod prtcl;
use crate::prtcl::{Message, RegisterPayload};
use crate::core::{Node};
use std::io::Write;
use std::net::{TcpStream, TcpListener, SocketAddr};
use std::str::FromStr;
use std::env;


fn register_node(server_sock: SocketAddr, client_sock: SocketAddr) -> std::io::Result<()>{
    
    let mut stream = match TcpStream::connect(server_sock) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let reg_msg : Message<String, String> = 
        Message::Register(RegisterPayload::new(client_sock.to_string()));

    let serialized_msg = serde_json::to_string(&reg_msg).unwrap();
    stream.write_all(serialized_msg.as_bytes()).unwrap();

    // stream.redad    

    Ok(())
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: ./client <server_ip:server_port> <client_ip:client_port>");
        std::process::exit(1);
    }

    // Parse server address
    let server_addr = match SocketAddr::from_str(&args[1]) {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to parse server address from {} with error: {}", args[1], e);
            std::process::exit(1); 
        }
    };
    let client_addr = match SocketAddr::from_str(&args[2]) {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to parse client address from {} with error: {}", args[2], e);
            std::process::exit(1);
        }
    };
    
    // Start client server
    let listener = match TcpListener::bind(client_addr) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to {}", client_addr);
            std::process::exit(1);
        }
    };
       
    // register a new node
    if let Err(e) = register_node(server_addr, client_addr) {
        eprintln!("Failed to register node: {}", e);
    }
}