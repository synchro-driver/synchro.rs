use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn handshake() {
    let stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    println!("Connected to server: {}", stream.peer_addr().unwrap());

    loop {
        // Receive the response from broadcaster
        let handshake_recvied: bool = false;
        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();

        while handshake_recvied {
            let bytes_read = reader.read_line(&mut buffer).unwrap();
            println!("bytes recv: {:?}", buffer);

            if bytes_read == 0 {
                // connection closed
                println!("connection closed by host");
                break;
            }
        }
    }
}
