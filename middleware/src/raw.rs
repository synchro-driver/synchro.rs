use std::ffi::CString;

use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, ValueOr};

// This contains all the data that are required to bind with ALSA api
#[derive(Debug)]
pub struct AlsaConfig {
    pub source: CString,
    pub format: Format,
    pub channel: u32,
    pub sample_rate: u32,
    pub frame_size: i64,
}

impl AlsaConfig {
    // TODO: handle unwrap
    pub fn new(source: &str, channel: u32, sample_rate: u32, frame_size: i64) -> AlsaConfig {
        let config: AlsaConfig = AlsaConfig {
            source: CString::new(source).unwrap(),
            format: Format::S16LE,
            channel,
            sample_rate,
            frame_size,
        };

        config
    }

    pub fn open_stream_buffer() -> [f64; 1024] {
        let buffer = [0.0f64; 1024];

        buffer
    }

    pub fn open_capture_device(source: CString) -> PCM {
        let pcm = PCM::new(source.to_str().unwrap(), Direction::Capture, false).unwrap();

        pcm
    }

    pub fn open_hardware_config(pcm: &PCM) -> HwParams {
        let hwp = HwParams::any(&pcm).unwrap();

        hwp
    }
}

pub struct AlsaStream<'a> {
    // Default size of frame is 1024
    pub stream: [f64; 1024],
    pub pcm: PCM,
    pub hw: HwParams<'a>,
}

impl AlsaStream<'_> {
    pub fn set_hardware_config(&mut self, config: &mut AlsaConfig) {
        self.hw.set_channels(config.channel).unwrap();
        self.hw
            .set_rate(config.sample_rate, ValueOr::Nearest)
            .unwrap();
        self.hw.set_format(Format::S16LE).unwrap();
        self.hw.set_access(Access::RWInterleaved).unwrap();
        self.hw
            .set_period_size_near(config.frame_size, ValueOr::Nearest)
            .unwrap();
    }

    pub fn attach_config_to_capture(&self) {
        self.pcm.hw_params(&self.hw).unwrap();
    }

    pub fn get_transfer_size(pcm: PCM) -> (u64, u64) {
        pcm.get_params().unwrap()
    }

    pub fn read_from_io(stream: &mut [f64], pcm: PCM) {
        let io_handler = pcm.io_f64().unwrap();

        match io_handler.readi(stream) {
            Ok(bytes) => println!("{} bytes read", bytes),
            Err(_) => {
                eprintln!("Overflow Occured");
                pcm.prepare().unwrap();
            }
        };
    }

    pub fn write_to_io(stream: &mut [f64], pcm: PCM) {
        let io_handler = pcm.io_f64().unwrap();

        match io_handler.writei(stream) {
            Ok(bytes) => println!("{} bytes flushed to IO", bytes),
            Err(_) => eprintln!("Flush Failed!"),
        }
    }

    pub fn close_capture_device(pcm: PCM) {
        pcm.drop().unwrap();
    }

    // use these for fetching the data for Protocol
    pub fn get_audio_format(hw: HwParams) -> Format {
        hw.get_format().unwrap()
    }

    pub fn get_audio_rate(hw: HwParams) -> u32 {
        hw.get_rate().unwrap()
    }

    pub fn get_period_size(hw: HwParams) -> i64 {
        hw.get_period_size().unwrap()
    }
}
