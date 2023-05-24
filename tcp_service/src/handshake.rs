use super::raw::{ClientLatencies, Host, MessageTypeIO};

// use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net;
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
pub async fn init(host: Host, _: &mut ClientLatencies) {
    let (start_io_tx, mut start_io_rx) = oneshot::channel::<MessageTypeIO>();
    let (stop_io_tx, mut stop_io_rx) = oneshot::channel::<MessageTypeIO>();
    let (client_tx, mut client_rx) = mpsc::unbounded_channel();

    println!("Starting as a host at port {}\n", host.port);
    let listener = net::TcpListener::bind(format!("{}:{}", host.ip, host.port))
        .await
        .expect("Expected server binding to port");

    // This thread waits for io input to start broadcast
    tokio::spawn(async move {
        let mut command = String::new();
        let mut run = true;
        let mut message_type = MessageTypeIO::Err;

        println!("[Usage] Type 'start' to Start streaming");
        println!("Type 'stop' to Stop streaming [Usage] \n");

        while run {
            // replace it with string check logic
            std::io::stdin()
                .read_line(&mut command)
                .expect("Failed to read line");

            if command.trim() == "start" {
                message_type = MessageTypeIO::Start;
            } else if command.trim() == "stop" {
                message_type = MessageTypeIO::Stop;
            } else {
                println!("Wrong Input!");
            }
            run = false;
        }

        match message_type {
            MessageTypeIO::Start => match start_io_tx.send(MessageTypeIO::Start) {
                Ok(_) => println!("Start message sent"),
                Err(_) => println!("send failed"),
            },
            MessageTypeIO::Stop => match stop_io_tx.send(MessageTypeIO::Start) {
                Ok(_) => println!("Stop message sent"),
                Err(_) => println!("send failed"),
            },
            MessageTypeIO::Err => println!("Failed to start the broadcast"),
        }
    });

    // This thread handles the switching of broadcast status
    tokio::spawn(async move {
        tokio::select! {
            message = &mut start_io_rx => {
                match message {
                    Ok(val) => if let MessageTypeIO::Start = val {
                        // add logic here
                        println!("Start broadcast");
                    },
                    Err(_) => eprintln!("Error in reciving start")
                };

            }
        }

        tokio::select! {
            message = &mut stop_io_rx => {
                match message {
                    Ok(val) => if let MessageTypeIO::Stop = val {
                        // add logic here
                        println!("Stopping broadcast");
                    },
                    Err(_) => eprintln!("Error in reciving stop")
                };
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
