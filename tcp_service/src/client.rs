use super::protocol_helpers::deserialize_handshake;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn handshake() {
    let stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    println!("Connected to server: {}", stream.peer_addr().unwrap());

    loop {
        // Receive the response from broadcaster
        let mut wating_handshake: bool = true;
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = vec![];

        while wating_handshake {
            let bytes_read = match reader.read_until(0, &mut buffer) {
                Ok(bytes) => {
                    if bytes == 15 {
                        wating_handshake = false;
                        bytes
                    } else {
                        0
                    }
                }
                Err(err) => {
                    eprintln!("{}", err);
                    0
                }
            };

            println!("bytes recv: {:?}:{}", buffer, bytes_read);
        }

        // deseraialize
        let handshake = deserialize_handshake(buffer);
        println!("Deser buff: {:?}", handshake);
    }
}
