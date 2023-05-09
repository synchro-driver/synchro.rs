use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    sender: String,
    content: String,
}

pub fn start() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    println!("Connected to server: {}", stream.peer_addr().unwrap());

    loop {
        print!("Enter a message: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Serialize the input message
        let message = Message {
            sender: "Client".to_string(),
            content: input.trim().to_string(),
        };
        let serialized_message = serde_json::to_string(&message).unwrap();

        // Send the serialized message to the server
        stream.write_all(serialized_message.as_bytes()).unwrap();

        // Receive the response from the server
        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer).unwrap();

        if bytes_read == 0 {
            // Connection closed
            println!("Connection closed by server");
            break;
        }

        // Deserialize the response message
        let response: Message = serde_json::from_str(&buffer).unwrap();
        println!(
            "Received response from {}: {}",
            response.sender, response.content
        );
    }
}
