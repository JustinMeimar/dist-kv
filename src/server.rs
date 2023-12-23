mod core;
use crate::core::{Node, Partitioner};
use std::net::{TcpListener, TcpStream, Ipv4Addr};

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
