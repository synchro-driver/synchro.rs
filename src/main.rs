// extern crate protocol;
// extern crate server;
// extern crate client;

use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

fn main() {
    // println!("{}", protocol::add(5, 6));

    let output_streams = OutputStream::try_default();
    let stream_handler = match output_streams {
        Ok((_, strean_handler)) => strean_handler,

        Err(_) => panic!("No inputs found!"),
    };

    let sink = Sink::try_new(&stream_handler).unwrap();

    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(2.0))
        .amplify(0.5);

    sink.append(source);
    sink.sleep_until_end();
}
