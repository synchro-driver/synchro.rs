use alsa::pcm::Format;
use serde::{Deserialize, Serialize};

// TODO: fix serde

// #[derive(Serialize, Deserialize)]
pub struct HandshakeProtocol<'a> {
    pub buffer_size: u16,
    pub source: &'a str,
    pub format: Format,
    pub rate: u32,
    pub channels: u32,
}

// #[derive(Serialize, Deserialize)]
pub struct StreamProtocol<'a> {
    // the actual size of the audio buffer that will be sent.
    // 0 <= size <= 1024
    pub buffer_size: u16,

    pub stream: &'a mut [f64],

    // used to check the order in which the packets arrive
    // similar to sliding window, use it with mod(size(u32)) to prevent overflows
    pub packet_flag: u32,
}

#[derive(Serialize, Deserialize)]
pub struct TerminateProtocol {
    pub fin: bool,
}
