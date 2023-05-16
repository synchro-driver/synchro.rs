use rmp_serde;
use serde::{Deserialize, Serialize};

// Set Format as pcm::Format::S16LE in reciver side
#[derive(Serialize, Deserialize)]
pub struct Handshake {
    pub buffer_size: u16,
    // pub source: &'a str,
    pub source: String,
    pub rate: u32,
    pub channels: u32,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct HandshakeResponse {
    pub latency: usize,
    pub name: String,
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

impl Handshake {
    pub fn new(
        buffer_size: u16,
        source: String,
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

    pub fn deserialize(buffer: Vec<u8>) -> Self {
        match rmp_serde::from_slice(&buffer) {
            Ok(val) => val,
            Err(_) => Self::new(0, "none".to_string(), 0, 0, 0),
        }
    }

    pub fn set_timestamp(&mut self, stamp: u64) {
        self.timestamp = stamp;
    }
}

impl HandshakeResponse {
    pub fn new(latency: usize, name: String) -> Self {
        HandshakeResponse { latency, name }
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

    pub fn deserialize(buffer: Vec<u8>) -> Self {
        match rmp_serde::from_slice(&buffer) {
            Ok(val) => val,
            Err(_) => Self::new(0, "error".to_string()),
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
    pub fn deserialize(buffer: Vec<u8>) -> Self {
        match rmp_serde::from_slice(&buffer) {
            Ok(val) => val,
            Err(_) => Self::new(0, 0, &[]),
        }
    }
}

impl StreamControl {
    pub fn new(state: bool) -> Self {
        StreamControl { alive: state }
    }

    pub fn kill(&mut self) {
        self.alive = false;
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

    pub fn deserialize(buffer: Vec<u8>) -> Self {
        match rmp_serde::from_slice(&buffer) {
            Ok(val) => val,
            Err(_) => Self::new(false),
        }
    }
}
