use std::collections::HashMap;
use std::fmt::Debug;
use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream};
use std::io::{self, Read};

pub struct Node<K, V> {
    data: HashMap<K, V>,
    cur_cap: usize,
    ip_addr: Ipv4Addr,
    port: u16,
}

impl<K, V> Node<K, V> 
where
    K: Eq + std::hash::Hash + Debug,
    V: Debug
{
    pub fn new(ip_addr: Ipv4Addr, port: u16) -> Self {
        Node { 
            data: HashMap::new(),
            cur_cap: 0,
            ip_addr: ip_addr, 
            port: port,
        }
    }

    pub fn kv_create(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    pub fn kv_update(&mut self, key: K, value: V) {
        self.data.entry(key).and_modify( |old_v| {
            *old_v = value;
        });
    }

    pub fn dump(&self) {
        for (key, value) in self.data.iter() {
            println!("{:?}:{:?}", key, value);
        }
    } 
}

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