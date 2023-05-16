use crate::raw;

// Setup functions
pub fn set_alsa_config(
    source: &str,
    channel: u32,
    sample_rate: u32,
    frame_size: i64,
) -> raw::AlsaConfig {
    raw::AlsaConfig::new(source, channel, sample_rate, frame_size)
}

pub fn set_alsa_stream(source: &str) -> raw::AlsaStream {
    raw::AlsaStream::new(source)
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

    // check for the termination condition
    while !terminate {
        raw::AlsaStream::read_from_io(stream);

        // write to protocol buffer
    }
}
