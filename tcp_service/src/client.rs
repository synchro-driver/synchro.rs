use super::protocol_helpers::{deserialize_handshake, get_serialized_handshake_responce};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

pub fn handshake(client_name: String, host_ip: String, host_port: u16) {
    let stream = TcpStream::connect(format!("{}:{}", host_ip, host_port)).unwrap();
    println!("Connected to server: {}", stream.peer_addr().unwrap());

    // Receive the response from broadcaster
    // let mut wating_handshake: bool = true;
    let mut reader = BufReader::new(&stream);
    let mut buffer: Vec<u8> = vec![];

    let bytes_read = match reader.read_until(0, &mut buffer) {
        Ok(bytes) => {
            if bytes == 15 {
                bytes
            } else {
                eprintln!("Wrong packet size recived!");
                0
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            0
        }
    };

    // send handshake responce
    let mut serialized_responce = [0u8; 16];
    let serialized_responce =
        get_serialized_handshake_responce(0, client_name.clone(), &mut serialized_responce);

    let mut writer = BufWriter::new(&stream);
    match writer.write(&serialized_responce) {
        Ok(bytes) => println!("{} bytes sent", bytes),
        Err(err) => eprintln!("Handshake responce failed: {}", err),
    }

    println!("bytes recv: {:?}:{}", buffer, bytes_read);

    let audio_params = deserialize_handshake(buffer);
    let mut audio_config = audio_params.get_audio_config();
    let mut audio_stream = audio_params.set_audio_stream();

    audio_params.config_client_audio(&mut audio_config, &mut audio_stream);
}
