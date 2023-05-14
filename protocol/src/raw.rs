use std::marker::PhantomData;

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
    pub stream: PhantomData<[f64]>,

    // used to check the order in which the packets arrive
    // similar to sliding window, use it with mod(size(u32)) to prevent overflows
    pub packet_flag: u32,
}

#[derive(Serialize, Deserialize)]
pub struct StreamInitiation {
    pub fin: bool,
}
