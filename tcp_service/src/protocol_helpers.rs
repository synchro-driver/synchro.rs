use protocol::raw;

use tokio::io::BufWriter;
use tokio::net::tcp::WriteHalf;

// Used to get a buffer writer with serialized handshake protocol
pub async fn handshake<'a>(write: &'a mut WriteHalf<'a>) -> BufWriter<&'a mut WriteHalf<'a>> {
    BufWriter::new(write)
}

// Used by client, to respond to handshake. This will be recived by the tokio::select!
// in broadcaster::init_handshake()
pub fn handshake_responce() {}

pub fn stream_init() -> Vec<u8> {
    raw::StreamControl::new(true).serialize()
}

pub fn stream_end() {}
