use middleware::raw::{AlsaConfig, AlsaStream};

use rmp_serde;
use serde::{Deserialize, Serialize};

// Set Format as pcm::Format::S16LE in reciver side
#[derive(Serialize, Deserialize, Debug)]
pub struct Handshake {
    pub buffer_size: u16,
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
    pub stream: Vec<u8>,

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

    pub fn default() -> Handshake {
        Handshake::new(0, "".to_string(), 0, 0, 0)
    }

    pub fn serialize(&self) -> Vec<u8> {
        // MessagePack impl
        match rmp_serde::to_vec(&self) {
            Ok(val) => val,
            Err(_) => {
                println!("serialization failed");
                Vec::new()
            }
        }
    }

    pub fn deserialize(buffer: Vec<u8>) -> Self {
        // MessagePack impl
        match rmp_serde::from_slice::<Self>(&buffer) {
            Ok(val) => {
                println!("Deserialize success");
                println!("val: {:?}", val);
                val
            }
            Err(err) => {
                eprintln!("Failed to deserialize: {}", err);
                Self::default()
            }
        }
    }

    pub fn set_timestamp(&mut self, stamp: u64) {
        self.timestamp = stamp;
    }

    pub fn set_rate(&mut self, rate: u32) {
        self.rate = rate;
    }

    pub fn set_channels(&mut self, channels: u32) {
        self.channels = channels;
    }

    pub fn get_audio_config(&self) -> AlsaConfig {
        AlsaConfig::new("default", self.channels, self.rate, 256)
    }

    pub fn get_audio_stream(&self) -> AlsaStream {
        AlsaStream::new("default", true)
    }

    pub fn set_audio_stream(&self) -> AlsaStream {
        AlsaStream::new("default", false)
    }

    pub fn config_client_audio(&self, config: &mut AlsaConfig, stream: &mut AlsaStream) {
        middleware::core::initialize_audio_paramters(config, stream);
    }
}

impl HandshakeResponse {
    pub fn new(latency: usize, name: String) -> Self {
        HandshakeResponse { latency, name }
    }

    pub fn default() -> Self {
        Self {
            latency: 0,
            name: "error".to_string(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        // MessagePack
        match rmp_serde::to_vec(&self) {
            Ok(val) => val,
            Err(_) => {
                println!("serialization failed");
                Vec::new()
            }
        }
    }

    pub fn deserialize(buffer: Vec<u8>) -> Self {
        // MessagePack
        match rmp_serde::from_slice::<Self>(&buffer) {
            Ok(val) => val,
            Err(_) => Self::new(0, "error".to_string()),
        }
    }
}

impl Stream {
    pub fn new(buffer_size: u16, packet_flag: u32, stream: &[u8]) -> Self {
        Stream {
            buffer_size,
            stream: stream.to_vec(),
            packet_flag,
        }
    }

    pub fn default() -> Self {
        Self {
            buffer_size: 0,
            stream: [0].to_vec(),
            packet_flag: 0,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        // MessagePack
        match rmp_serde::to_vec(&self) {
            Ok(val) => val,
            Err(_) => {
                println!("serialization failed");
                Vec::new()
            }
        }
    }
    pub fn deserialize(buffer: Vec<u8>) -> Self {
        // MessagePack
        match rmp_serde::from_slice::<Self>(&buffer) {
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
