use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpStream;

pub struct Host {
    pub ip: String,
    pub port: u16,
    pub broadcast_capacity: usize,
    pub clients: HashMap<SocketAddr, TcpStream>,
}

impl Host {
    pub fn new(ip: String, port: u16, broadcast_capacity: usize) -> Self {
        Self {
            ip,
            port,
            broadcast_capacity,
            clients: HashMap::new(),
        }
    }

    pub fn add_clients(&mut self, client: SocketAddr, stream: TcpStream) {
        self.clients.insert(client, stream);
    }
}
