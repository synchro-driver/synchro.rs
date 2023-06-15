use super::protocol_helpers::get_serialized_handshake;
use super::raw::{ClientLatencies, Host, MessageTypeIO};

use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net;
use tokio::sync::oneshot;

#[tokio::main]
pub async fn init(host: Host, clients: &mut ClientLatencies) {
    let (start_io_tx, mut start_io_rx) = oneshot::channel::<MessageTypeIO>();
    let (stop_io_tx, _) = oneshot::channel::<MessageTypeIO>();

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

        // Uncomment after stop logic is implemented
        // tokio::select! {
        //     message = &mut stop_io_rx => {
        //         match message {
        //             Ok(val) => if let MessageTypeIO::Stop = val {
        //                 // add logic here
        //                 println!("Stopping broadcast");
        //             },
        //             Err(_) => eprintln!("Error in reciving stop")
        //         };
        //     }
        // }
    });

    loop {
        tokio::select! {
            client = listener.accept() => {
                println!("New client joined");

                let (mut socket, addr) = client.unwrap();
                clients.add_clients(addr);

                // This thread handles handshaking with client
                tokio::spawn(async move {
                    let (read, mut write) = socket.split();
                    let mut tcp_reader = BufReader::new(read);
                    let mut responce_buffer = String::new();

                    // send the handshake request
                    let mut serilized_handshake = [0u8; 16];
                    let serilized_handshake = get_serialized_handshake(64, "default".to_string(), 44100, 1, &mut serilized_handshake).await;

                    println!("serilized_handshake: {:?}", serilized_handshake);
                    write.write_all(&serilized_handshake).await.unwrap();

                    // TODO: validate this for proper error handling
                    // match write.ready(tokio::io::Interest::WRITABLE).await {
                    //     Ok(_) => {
                    //         // let mut write_buffer = BufWriter::new(write);
                    //         // write_buffer.write_all(serilized_handshake).await.unwrap();
                    //         println!("Send handshake");
                    //     },
                    //     Err(_) => {
                    //         // implemented retry logic
                    //         eprintln!("Handshake send broke...");
                    //     }
                    // }

                    // accept handshake responce
                    tokio::select! {
                        res = tcp_reader.read_line(&mut responce_buffer) => {
                            let message = res.unwrap();
                            println!("{}", message);
                        }
                    }
                });
                }

            // TODO
            // Handle io intrupt to start streamm
        }
    }

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
