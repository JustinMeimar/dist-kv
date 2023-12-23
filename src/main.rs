use std::collections::HashMap;
use std::fmt::Debug;
use std::net::{IpAddr, Ipv4Addr};

struct Node<K, V> {
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
    fn new(ip_addr: Ipv4Addr, port: u16) -> Self {
        Node { 
            data: HashMap::new(),
            cur_cap: 0,
            ip_addr: ip_addr, 
            port: port,
        }
    }

    fn kv_create(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    fn kv_update(&mut self, key: K, value: V) {
        self.data.entry(key).and_modify( |old_v| {
            *old_v = value;
        });
    }

    fn dump(&self) {
        for (key, value) in self.data.iter() {
            println!("{:?}:{:?}", key, value);
        }
    } 
}

struct ShardManager<K, V> {
    nodes: Vec<Node<K, V>>,

}

fn main() {

    let ip_addr : Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let port: u16 = 8081;
    let mut node: Node<String, String> = Node::new(ip_addr, port);

    node.kv_create("key1".to_string(), "8".to_string());
    node.dump();
}
