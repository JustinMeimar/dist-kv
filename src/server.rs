mod core;
mod prtcl;
use crate::prtcl::*;
use crate::core::{Node};
use std::net::{TcpListener, TcpStream};
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use serde_json::{Error as SerdeError};
use std::io::{self, Read, Write};
use std::fmt::Debug;

pub struct Partitioner<K, V> {
    nodes: Vec<Node<K, V>>,
    listener: TcpListener, 
}

impl<K, V> Partitioner<K, V>
where
    K: Serialize + DeserializeOwned + Debug,
    V: Serialize + DeserializeOwned + Debug,
{    
    pub fn new(port: u16) -> Self {
        let address = format!("localhost:{}", port);
        Partitioner {
            nodes: Vec::new(),
            listener: TcpListener::bind(address)
                .expect("Failed to bind"),
        }
    }
 
    fn handle_put(&self, payload: PutPayload<K, V>) -> io::Result<()>{
        Ok(())
    }
    fn handle_get(&self, payload: GetPayload<K>) -> io::Result<()>{
        Ok(())
    }
    fn handle_delete(&self, payload: DeletePayload<K>) -> io::Result<()>{
        Ok(())
    }
    fn handle_response(&self, payload: ResponsePayload) -> io::Result<()>{
        Ok(())
    }
    
    fn handle_register(&self, stream: &mut TcpStream, payload: RegisterPayload) -> io::Result<()>{
        
        println!("recieved register message: {:?}", payload);
        let res = String::new();

        stream.write_all(res.as_bytes())?;
        stream.flush()?;

        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) -> io::Result<()> {
        loop {  
            let mut buf = String::new();
            
            {
                let mut reader = io::BufReader::new(&mut stream);
                let bytes_read = reader.read_to_string(&mut buf)?;
                if bytes_read == 0 {
                    break;
                }
            } 

            let data: Result<Message<K, V>, SerdeError> = serde_json::from_str(&buf);

            let _ = match data {
                Ok(Message::Put(payload)) => self.handle_put(payload),
                Ok(Message::Get(payload)) => self.handle_get(payload),
                Ok(Message::Delete(payload)) => self.handle_delete(payload),
                Ok(Message::Register(payload)) => self.handle_register(&mut stream, payload),
                Ok(Message::Response(payload)) => self.handle_response(payload),
                Err(e) => return Err(e.into())
            };
        }
        Ok(())
    }

    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    match self.handle_client(stream) {
                        Ok(_) => {},
                        Err(e)=> eprintln!("Error: {}", e)
                    } 
                }
                Err(e) => eprintln!("Connection failed: {}", e)
            }
        } 
    }
}
fn main() {
    let port: u16 = 8080;
    let partitioner: Partitioner<String, String> = Partitioner::new(port);
    partitioner.listen();
}
