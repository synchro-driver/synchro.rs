use super::protocol;
use super::raw::{ClientLatencies, Host};

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net;
// use tokio::sync::oneshot;

#[derive(Serialize, Deserialize, Debug)]
struct Packet {
    content: String,
    timestamp: i64,
}

#[tokio::main]
pub async fn init(host: Host, clients: &mut ClientLatencies) {
    println!("Starting as a host at port {}", host.port);

    let listener = net::TcpListener::bind(format!("{}:{}", host.ip, host.port))
        .await
        .unwrap();

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        clients.add_clients(addr);

        // spawn new thread to initiate protocol handshake
        tokio::spawn(async move {
            println!("New thread spawned");
            let (read, mut write) = socket.split();

            // Here the writer should contain serilized handshake protocol
            let tcp_writer = protocol::handshake(&mut write).await;
            println!("{:?}", tcp_writer);

            let mut tcp_reader = BufReader::new(read);
            let mut responce_buffer = String::new();

            // Handle client events
            loop {
                tokio::select! {
                    res = tcp_reader.read_line(&mut responce_buffer) => {
                        println!("{}", res.unwrap());
                    }
                }
            }
        });

        // let (tx, _rx) = sync::broadcast::channel(host.broadcast_capacity);
        // let tx = tx.clone();
        // let mut rx = tx.subscribe();

        // tokio::spawn(async move {
        // let (read, mut write) = socket.split();
        //
        // let mut reader = BufReader::new(read);
        //
        // let mut line = String::new();
        //
        // });
    }
}
