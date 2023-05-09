use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    sender: String,
    content: String,
}

fn handle_client(stream: TcpStream) {
    let mut stream_clone = stream.try_clone().expect("clone failed...");

    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        let bytes_read = reader.read_line(&mut buffer).unwrap();

        if bytes_read == 0 {
            // Connection closed
            println!("Connection closed: {}", stream.peer_addr().unwrap());
            break;
        }

        // Deserialize the received message
        let message: Message = serde_json::from_str(&buffer).unwrap();
        println!(
            "Received message from {}: {}",
            message.sender, message.content
        );

        // Echo back the received message
        let response = Message {
            sender: "Server".to_string(),
            content: message.content,
        };
        let serialized_response = serde_json::to_string(&response).unwrap();
        stream_clone
            .write_all(serialized_response.as_bytes())
            .unwrap();
    }
}

pub fn start() {
    const PORT: i32 = 8000;

    println!("Starting as a host at port {}", PORT);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();

    // Handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // Spawn a new thread for each incoming connection
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
