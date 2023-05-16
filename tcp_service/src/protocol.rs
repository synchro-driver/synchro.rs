// use crate::protocol;

use tokio::io::BufWriter;
use tokio::net::tcp::WriteHalf;

// Used to get a buffer writer with serialized handshake protocol
pub async fn handshake<'a>(write: &'a mut WriteHalf<'a>) -> BufWriter<&'a mut WriteHalf<'a>> {
    BufWriter::new(write)
}

pub fn handshake_responce() {}

pub fn stream_init() {}

pub fn stream_end() {}
