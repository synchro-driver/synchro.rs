use rmp_serde;
use serde::{Deserialize, Serialize};

// Set Format as pcm::Format::S16LE in reciver side
#[derive(Serialize, Deserialize)]
pub struct Handshake<'a> {
    pub buffer_size: u16,
    pub source: &'a str,
    pub rate: u32,
    pub channels: u32,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct HandshakeResponce {
    pub latency: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Stream {
    // the actual size of the audio buffer that will be sent.
    // 0 <= size <= 1024
    pub buffer_size: u16,

    // pub stream: &'a mut [f64],
    // pub stream: [f64; 1024],
    pub stream: Vec<f64>,

    // used to check the order in which the packets arrive
    // similar to sliding window, use it with mod(size(u32)) to prevent overflows
    pub packet_flag: u32,
}

#[derive(Serialize, Deserialize)]
pub struct StreamControl {
    pub alive: bool,
}

// TODO: Handle deserialize code inside impl blocks

impl Handshake<'_> {
    pub fn new(
        buffer_size: u16,
        source: &str,
        rate: u32,
        channels: u32,
        timestamp: u64,
    ) -> Handshake {
        Handshake {
            buffer_size,
            source,
            rate,
            channels,
            timestamp,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match rmp_serde::to_vec(&self) {
            Ok(val) => val,
            Err(_) => {
                println!("serialization failed");
                Vec::new()
            }
        }
    }
}

impl HandshakeResponce {
    pub fn new(latency: usize) -> Self {
        HandshakeResponce { latency }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match rmp_serde::to_vec(&self) {
            Ok(val) => val,
            Err(_) => {
                println!("serialization failed");
                Vec::new()
            }
        }
    }
}

impl Stream {
    pub fn new(buffer_size: u16, packet_flag: u32, stream: &[f64]) -> Self {
        Stream {
            buffer_size,
            stream: stream.to_vec(),
            packet_flag,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match rmp_serde::to_vec(&self) {
            Ok(val) => val,
            Err(_) => {
                println!("serialization failed");
                Vec::new()
            }
        }
    }
}

impl StreamControl {
    pub fn new(state: bool) -> Self {
        StreamControl { alive: state }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match rmp_serde::to_vec(&self) {
            Ok(val) => val,
            Err(_) => {
                println!("serialization failed");
                Vec::new()
            }
        }
    }
}
