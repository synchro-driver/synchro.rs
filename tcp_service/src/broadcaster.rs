use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net, sync,
};

#[derive(Serialize, Deserialize, Debug)]
struct Packet {
    content: String,
    timestamp: i64,
}

pub struct Host {
    pub ip: String,
    pub port: u16,
    pub broadcast_capacity: usize,
}

impl Host {
    pub fn new(ip: String, port: u16, broadcast_capacity: usize) -> Self {
        Self {
            ip,
            port,
            broadcast_capacity,
        }
    }
}

#[tokio::main]
pub async fn init(metadata: Host) {
    println!("Starting as a host at port {}", metadata.port);

    let listener = net::TcpListener::bind(format!("{}:{}", metadata.ip, metadata.port))
        .await
        .unwrap();

    let (tx, _rx) = sync::broadcast::channel(metadata.broadcast_capacity);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (read, mut write) = socket.split();

            let mut reader = BufReader::new(read);

            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line)=>{
                        if result.unwrap()==0{
                            break;
                        }

                        let now = Utc::now();

                        let message = Packet {
                            content:line.clone(),
                            timestamp: now.timestamp_millis()
                        };

                        tx.send((format!("{}\n",serde_json::to_string(&message).unwrap()), addr)).unwrap();
                        line.clear();
                    }

                    result = rx.recv()=>{
                        let (msg, other_addr) = result.unwrap();

                        if addr !=other_addr{
                        write.write_all(msg.as_bytes()).await.unwrap();
                    }
                    }
                }
            }
        });
    }
}
