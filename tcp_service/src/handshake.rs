use super::raw::{ClientLatencies, Host, MessageTypeIO};

// use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net;
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
pub async fn init(host: Host, _: &mut ClientLatencies) {
    let (io_tx, mut io_rx) = oneshot::channel::<MessageTypeIO>();
    let (client_tx, mut client_rx) = mpsc::unbounded_channel();

    // This thread accepts incomming clients
    // 1. It also sends the socket data of client to another thead to handle handshake
    // 2. It also listens to io_rx to start stream
    println!("Starting as a host at port {}\n", host.port);
    let listener = net::TcpListener::bind(format!("{}:{}", host.ip, host.port))
        .await
        .expect("Expected server binding to port");

    println!("[Usage] Type 'start' to Start streaming");
    println!("Type 'stop' to Stop streaming [Usage] \n");

    // This thread waits for io input to start broadcast
    tokio::spawn(async move {
        let mut command = String::new();
        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        if command == "".to_string() {
            io_tx.send(MessageTypeIO::Start).unwrap();
        } else if command == "stop".to_string() {
            io_tx.send(MessageTypeIO::Stop).unwrap();
        }
    });

    // This thread handles the switching of broadcast status
    tokio::spawn(async move {
        loop {
            tokio::select! {
                message = &mut io_rx => {
                    // handle broadcase start and stop logic
                    let status = match message {
                        Ok(val) => val,
                        Err(_) => MessageTypeIO::Err
                    };

                    match status {
                        MessageTypeIO::Start => println!("Start broadcast"),
                        MessageTypeIO::Stop => println!("Stop broadcast"),
                        MessageTypeIO::Err => eprintln!("IO error")
                    };
                }
            }
        }
    });

    loop {
        tokio::select! {
            client = listener.accept() => {
                println!("new client");
                let (socket, _) = client.unwrap();
                // clients.add_clients(addr);

                client_tx.send(socket).unwrap();
            }

            // TODO
            // Handle responce from client
            // Handle io intrupt to start streamm

        }
    }

    // This thread handles handshaking with client
    // tokio::spawn(async move {
    //     let (read, mut write) = client_rx.recv().await.unwrap().split();
    //     let mut tcp_reader = BufReader::new(read);
    //     let mut responce_buffer = String::new();
    //
    //     loop {
    //         tokio::select! {
    //             res = tcp_reader.read_line(&mut responce_buffer) => {
    //                 let message = res.unwrap();
    //                 println!("{}", message);
    //             }
    //         }
    //     }
    // });

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
    // }
}
