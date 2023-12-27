use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="method", content="payload")]
pub enum Message<K,V> {
    Put(PutPayload<K,V>),
    Get(GetPayload<K>),
    Delete(DeletePayload<K>),
    Register(RegisterPayload),
    Response(ResponsePayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PutPayload<K, V> {
    pub key: K,
    pub value: V
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPayload<K> {
    pub key: K
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletePayload<K> {
    pub key: K
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterPayload {
    pub client_ip: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsePayload {
    pub message: String,    // which message is this response for
    pub status: String      // HTTP status
}

impl RegisterPayload {
    pub fn new(client_ip: String) -> Self {
        RegisterPayload { client_ip }
    }
}

impl<K, V> PutPayload<K, V> {
    pub fn new(key: K, value: V) -> Self {
        PutPayload { key, value }
    }
}

impl<K> GetPayload<K> {
    pub fn new(key: K) -> Self {
        GetPayload { key }
    }
}

impl<K> DeletePayload<K> {
    pub fn new(key: K) -> Self {
        DeletePayload { key }
    }
}

impl ResponsePayload {
    pub fn new(message: String, status: String) -> Self {
        ResponsePayload { message, status }
    }
}