use protocol::raw;

// Used to get a buffer writer with serialized handshake protocol
pub async fn get_serialized_handshake<'a>(
    buffer_size: u16,
    source: String,
    rate: u32,
    channel: u32,
    // need to update this with std::Box for dynamic header packet sizes
    serial_buffer: &'a mut [u8; 16],
) -> &'a [u8] {
    let mut handshake_packet = raw::Handshake::new(buffer_size, source, 0, 0, 0);
    handshake_packet.set_rate(rate);
    handshake_packet.set_channels(channel);

    for byte in handshake_packet.serialize().iter().enumerate() {
        serial_buffer[byte.0] = byte.1.to_owned();
    }

    let hand = raw::Handshake::default();
    hand.deserialize(serial_buffer.to_vec());
    println!("deser: {:?}", hand);
    serial_buffer
}

pub fn deserialize_handshake(buffer: Vec<u8>) -> raw::Handshake {
    let handshake = raw::Handshake::default();
    handshake.deserialize(buffer);

    handshake
}

// Used by client, to respond to handshake. This will be recived by the tokio::select!
// in broadcaster::init_handshake()
pub fn handshake_responce() {}

pub fn stream_init() -> Vec<u8> {
    raw::StreamControl::new(true).serialize()
}

pub fn stream_end() {}
