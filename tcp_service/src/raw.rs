use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Debug)]
pub enum MessageTypeIO {
    Start,
    Stop,
    Err,
}

pub struct Host {
    pub ip: String,
    pub port: u16,
    pub broadcast_capacity: usize,
    pub clients: Vec<SocketAddr>,
}

pub struct ClientLatencies {
    pub clients: HashMap<SocketAddr, usize>,
}

impl Host {
    pub fn new(ip: String, port: u16, broadcast_capacity: usize) -> Self {
        Self {
            ip,
            port,
            broadcast_capacity,
            clients: Vec::new(),
        }
    }
}

impl ClientLatencies {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn add_clients(&mut self, client: SocketAddr) {
        self.clients.insert(client, 0);
    }

    // Fix this
    // pub fn update_latency(&mut self, client: SocketAddr, latency: &mut usize) {
    //     self.clients.get_mut(client).unwrap() = latency;
    // }
}
