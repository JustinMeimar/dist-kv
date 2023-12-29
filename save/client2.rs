mod core;
mod prtcl;
use crate::prtcl::{Message, RegisterPayload};
use crate::core::{Node};
use std::io::{Write, Read};
use std::net::{TcpStream, TcpListener, SocketAddr};
use std::str::FromStr;
use std::env;
use std::thread;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
// fn register_node(server_sock: SocketAddr, client_sock: SocketAddr) -> std::io::Result<()>{
    
//     let mut stream = match TcpStream::connect(server_sock) {
//         Ok(s) => s,
//         Err(e) => return Err(e),
//     };

//     let mut buf = [0; 1024];
//     let reg_msg : Message<String, String> = 
//         Message::Register(RegisterPayload::new(client_sock.to_string()));

//     let serialized_msg = serde_json::to_string(&reg_msg).unwrap();
//     stream.write_all(serialized_msg.as_bytes()).unwrap();

//     // std::thread::sleep(std::time::Duration::from_secs(1));   

//     // println!("starting read");
//     // let bytes_read = stream.read(&mut buf).unwrap();
//     // let response = String::from_utf8_lossy(&buf[..bytes_read]);
//     // println!("response: {}", response); 
    
//     Ok(())
// }

// use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream};
// use serde::Deserialize;
// use std::io::{self, Read};

pub struct ClientNode<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> ClientNode<K, V> 
where
    K: Eq + std::hash::Hash + Debug,
    V: Debug
{
    pub fn new() -> Self {
        ClientNode { 
            data: HashMap::new(),
        }
    }

    pub fn spin(&self, client_addr: SocketAddr) {
        let handler = thread::spawn(move || {
            let listener = TcpListener::bind(client_addr)
                .expect("failed to bind");
        });   
    }

    pub fn connect(&self, server_addr: SocketAddr) {
        let handler = thread::spawn(move || {
            let mut stream = TcpStream::connect(server_addr);
        });
    }
}

fn main() -> io::Result<()>{

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: ./client <server_ip:port> <client_ip:port>");
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

    let reg_msg : Message<String, String> = 
        Message::Register(RegisterPayload::new(client_addr.to_string()));
    let serialized_msg = serde_json::to_string(&reg_msg)?;

    let mut stream = TcpStream::connect(server_addr).unwrap();
    stream.write_all(serialized_msg.as_bytes())?;

    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    println!("Received response: {}", String::from_utf8_lossy(&buffer));
    
    Ok(())
    // let node : ClientNode<String, String> = ClientNode::new();
    // node.spin(client_addr);
    // node.connect(server_addr);

    // Start client server
    // let listener = match TcpListener::bind(client_addr) {
    //     Ok(listener) => listener,
    //     Err(e) => {
    //         eprintln!("Failed to bind to {}", client_addr);
    //         std::process::exit(1);
    //     }
    // };
       
    // // register a new node
    // let handler = thread::spawn(move || {
    //     // register_node(server_addr, client_addr)
    //     spin_client();
    // });
   
    // match handler.join() {
    //     Ok(Ok(())) => {
    //         println!("Node registered successfully");
    //     }
    //     Ok(Err(e)) => {
    //         eprintln!("Error in register_node: {}", e);
    //     }
    //     Err(e) => {
    //         eprintln!("Thread panicked: {:?}", e);
    //     }
    // }; 
}