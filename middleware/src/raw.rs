use std::ffi::CString;

// This contains all the data that are required to bind with ALSA api
#[derive(Debug)]
pub struct AlsaConfig {
    pub source: CString,
    pub format: u8,
    pub channel: u32,
    pub sample_rate: u32,
    pub frames: i64,
}

impl AlsaConfig {
    // TODO: handle unwrap
    pub fn new(
        source: &str,
        format: u8,
        channel: u32,
        sample_rate: u32,
        frames: i64,
    ) -> AlsaConfig {
        let config: AlsaConfig = AlsaConfig {
            source: CString::new(source).unwrap(),
            format,
            channel,
            sample_rate,
            frames,
        };

        config
    }
}

#[derive(Debug)]
pub struct AlsaStream {
    pub stream: *mut f64,
}
