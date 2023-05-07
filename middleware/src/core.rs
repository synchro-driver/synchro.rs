use crate::raw;

pub fn initialize_audio_paramters(config: &mut raw::AlsaConfig) {
    // implement pcm clone if needed
    let pcm = raw::AlsaConfig::open_capture_device(config.source.clone());

    let mut stream = raw::AlsaStream {
        stream: raw::AlsaConfig::open_stream_buffer(),
        pcm: raw::AlsaConfig::open_capture_device(config.source.clone()),
        hw: raw::AlsaConfig::open_hardware_config(&pcm),
    };

    raw::AlsaStream::set_hardware_config(&mut stream, config);

    raw::AlsaStream::attach_config_to_capture(&stream);

    let actual_format = raw::AlsaStream::get_audio_format(stream.hw.clone());
    let actual_rate = raw::AlsaStream::get_audio_rate(stream.hw.clone());
    let actual_period_size = raw::AlsaStream::get_period_size(stream.hw.clone());

    config.format = actual_format;
    config.sample_rate = actual_rate;
    config.frame_size = actual_period_size;
}

pub fn thread_hijack() {}
