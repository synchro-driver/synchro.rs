use super::protocol;
use super::raw::Host;

use serde::{Deserialize, Serialize};
use tokio::net;

#[derive(Serialize, Deserialize, Debug)]
struct Packet {
    content: String,
    timestamp: i64,
}

#[tokio::main(worker_threads = 1)]
pub async fn init(host: &mut Host) {
    println!("Starting as a host at port {}", host.port);

    let listener = net::TcpListener::bind(format!("{}:{}", host.ip, host.port))
        .await
        .unwrap();

    // let (tx, _rx) = sync::broadcast::channel(host.broadcast_capacity);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();

        host.add_clients(addr, socket);

        // spawn new thread to initiate protocol handshake
        // tokio::spawn(move || protocol::handshake());

        // let tx = tx.clone();
        // let mut rx = tx.subscribe();

        // tokio::spawn(async move {
        // let (read, mut write) = socket.split();
        //
        // let mut reader = BufReader::new(read);
        //
        // let mut line = String::new();
        // });
    }
}
