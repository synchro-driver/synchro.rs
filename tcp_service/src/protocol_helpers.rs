use protocol::raw;

use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::tcp::WriteHalf;

// Used to get a buffer writer with serialized handshake protocol
pub async fn handshake<'a>(write: &'a mut WriteHalf<'a>) -> BufWriter<&'a mut WriteHalf<'a>> {
    let mut write_buffer = BufWriter::new(write);

    for byte in raw::Handshake::new(1024, "default".to_string(), 0, 0, 0)
        .serialize()
        .iter()
        .enumerate()
    {
        write_buffer.write_u8(byte.1.to_owned());
    }

    println!("{:?}", write_buffer);
    write_buffer
}

// Used by client, to respond to handshake. This will be recived by the tokio::select!
// in broadcaster::init_handshake()
pub fn handshake_responce() {}

pub fn stream_init() -> Vec<u8> {
    raw::StreamControl::new(true).serialize()
}

pub fn stream_end() {}
