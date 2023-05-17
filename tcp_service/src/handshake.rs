use super::protocol_helpers;
use super::raw::{ClientLatencies, Host};

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net;
use tokio::sync::oneshot;

#[derive(Serialize, Deserialize, Debug)]
struct Packet {
    content: String,
    timestamp: i64,
}

#[tokio::main]
pub async fn server(host: Host, clients: &mut ClientLatencies) {
    println!("Starting as a host at port {}", host.port);

    let listener = net::TcpListener::bind(format!("{}:{}", host.ip, host.port))
        .await
        .unwrap();

    // here oneshot is used to send a message to broadcast setup StreamReady signal
    // broadcast is used to broadcast the stream and stream life to the clients

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        clients.add_clients(addr);

        // used to send signal to start stream transfer
        let (io_tx, mut io_rx) = oneshot::channel::<Vec<u8>>();

        tokio::spawn(async move {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            io_tx.send(protocol_helpers::stream_init()).unwrap();
        });

        // spawn new thread to initiate protocol handshake
        tokio::spawn(async move {
            println!("New thread spawned");
            let (read, mut write) = socket.split();

            // Here the writer should contain serilized handshake protocol
            let tcp_writer = protocol_helpers::handshake(&mut write).await;
            println!("{:?}", tcp_writer);

            let mut tcp_reader = BufReader::new(read);
            let mut responce_buffer = String::new();

            // Handle client events
            loop {
                tokio::select! {
                    res = tcp_reader.read_line(&mut responce_buffer) => {
                        let message = res.unwrap();
                        println!("{}", message);
                    }

                    _ = &mut io_rx => {
                        println!("Starting streaming");

                        // send StreamControl protocol broadcast
                    }

                    // TODO
                    // Handle responce from client
                    // Handle io intrupt to start streamm
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
