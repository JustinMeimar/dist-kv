use std::collections::HashMap;
use std::fmt::Debug;
use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream};
use serde::Deserialize;
use std::io::{self, Read};

pub struct Node<K, V> {
    data: HashMap<K, V>,
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
            ip_addr: ip_addr, 
            port: port,
        }
    }

    pub fn kv_get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
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