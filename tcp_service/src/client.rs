use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn start() {
    let stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    println!("Connected to server: {}", stream.peer_addr().unwrap());

    loop {
        // Receive the response from broadcaster
        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer).unwrap();

        if bytes_read == 0 {
            // Connection closed
            println!("Connection closed by host");
            break;
        }

        // Deserialize the packet message
    }
}
