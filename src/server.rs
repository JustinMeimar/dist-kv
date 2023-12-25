mod core;
use crate::core::{Node};
use std::net::{TcpListener, TcpStream, Ipv4Addr};
use std::io::Read;
use serde::{Serialzier};
// use serde::{Seri}
pub struct Partitioner<K, V> {
    nodes: Vec<Node<K, V>>,
    listener: TcpListener, 
    max_nodes: usize,
}

impl<K, V> Partitioner<K, V>
{    
    pub fn new(port: u16) -> Self {
        let address = format!("127.0.0.1:{}", port);
        Partitioner {
            nodes: Vec::new(),
            max_nodes: 0x4000,
            listener: TcpListener::bind(address)
                .expect("Failed to bind"),
        }
    }

    pub fn parse_message() {
        
    }

    pub fn listen(&self) {
        println!("listening"); 
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buf = [0; 1024];
                    stream.read(&mut buf).expect("failed to react");
                    println!("Recieved: {}", String::from_utf8_lossy(&buf));
                }
                Err(e) => eprintln!("Connection failed: {}", e)
            }
        } 
    }

    fn register_node() {

    }

    fn create(&self, key: K, value: V) {
    }
}
fn main() {

    let ip_addr : Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let port: u16 = 8081;
    let mut node: Node<String, String> = Node::new(ip_addr, port);

    node.kv_create("key1".to_string(), "8".to_string());
    node.kv_create("key2".to_string(), "28".to_string());
    node.dump();
    
    node.kv_update("key1".to_string(), "9".to_string());
    node.dump();

    let partitioner: Partitioner<String, String> = Partitioner::new(8080);
    partitioner.listen();

}
