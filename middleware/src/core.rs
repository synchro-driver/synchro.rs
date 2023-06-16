use crate::raw;
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Setup functions
pub fn set_alsa_config(
    source: &str,
    channel: u32,
    sample_rate: u32,
    frame_size: i64,
) -> raw::AlsaConfig {
    raw::AlsaConfig::new(source, channel, sample_rate, frame_size)
}

pub fn set_alsa_stream(source: &str, is_input: bool) -> raw::AlsaStream {
    raw::AlsaStream::new(source, is_input)
}

// Helper function
pub fn initialize_audio_paramters(config: &mut raw::AlsaConfig, stream: &mut raw::AlsaStream) {
    let hw = raw::AlsaStream::open_hardware_config(&stream);

    raw::AlsaStream::set_hardware_config(&hw, config);

    raw::AlsaStream::attach_config_to_capture(&stream, &hw);

    let actual_format = raw::AlsaStream::get_audio_format(hw.clone());
    let actual_rate = raw::AlsaStream::get_audio_rate(hw.clone());
    let actual_period_size = raw::AlsaStream::get_period_size(hw.clone());

    config.format = actual_format;
    config.sample_rate = actual_rate;
    config.frame_size = actual_period_size;
}

// Entry point
pub fn io_read(config: &mut raw::AlsaConfig, stream: &mut raw::AlsaStream, terminate: bool) {
    initialize_audio_paramters(config, stream);

    // let (buffer_size, period_size) = raw::AlsaStream::get_transfer_size(stream);

    let mut file = File::create("array.txt").expect("Failed to create file");

    // check for the termination condition
    while !terminate {
        raw::AlsaStream::read_from_io(stream);

        // to write to file
        writeln!(file, "{:?}", stream.stream).expect("Failed to write to file");

        // write to protocol buffer
    }
}

// Exit point
pub fn io_write(config: &mut raw::AlsaConfig, stream: &mut raw::AlsaStream, terminate: bool) {
    initialize_audio_paramters(config, stream);

    // let (buffer_size, period_size) = raw::AlsaStream::get_transfer_size(stream);

    // let mut file = File::create("array.txt").expect("Failed to create file");

    // check for the termination condition
    // while !terminate {
    //     raw::AlsaStream::write_to_io(stream);

    //     // to write to file
    //     // writeln!(file, "{:?}", stream.stream).expect("Failed to write to file");

    //     // write to protocol buffer
    // }

    let file = File::open("array.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let read_array: [i16; 1024] = [0; 1024];
    for (i, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            // println!("Line {}: {}", i, line);

            let input_array: Vec<i16> = match serde_json::from_str(&line) {
                Ok(v) => v,
                Err(e) => {
                    println!("Failed to parse line {}", e);
                    vec![]
                }
            };

            // println!("Input array: {:?}", input_array);
            let c = input_array.as_slice();

            raw::AlsaStream::write_to_io(stream, c);
        } else {
            panic!("Failed to read line {}", i);
        }
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
