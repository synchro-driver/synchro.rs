use std::ffi::CString;

use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, ValueOr};

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

pub struct AlsaStream {
    pub stream: *mut [f64],
    pub pcm: PCM,
}

impl AlsaStream {
    pub fn open_capture_device(source: &str) -> PCM {
        let pcm = PCM::new(source, Direction::Capture, false).unwrap();

        pcm
    }

    pub fn open_hardware_config(pcm: &PCM) -> HwParams {
        let hwp = HwParams::any(&pcm).unwrap();

        hwp
    }

    pub fn set_hardware_config(hwp: &mut HwParams, config: AlsaConfig) {
        hwp.set_channels(config.channel).unwrap();
        hwp.set_rate(config.sample_rate, ValueOr::Nearest).unwrap();
        hwp.set_format(Format::S16LE).unwrap();
        hwp.set_access(Access::RWInterleaved).unwrap();
        hwp.set_period_size_near(config.frames, ValueOr::Nearest)
            .unwrap();
    }

    pub fn attach_config_to_capture(hwp: &HwParams, pcm: &PCM) {
        pcm.hw_params(&hwp).unwrap();
    }

    pub fn get_transfer_size(pcm: PCM) -> (u64, u64) {
        pcm.get_params().unwrap()
    }

    pub fn read_from_io(stream: &mut [f64], pcm: PCM) {
        let io_handler = pcm.io_f64().unwrap();

        match io_handler.readi(stream) {
            Ok(bytes) => {
                println!("{} transfered", bytes)
            }
            Err(_) => {
                pcm.prepare().unwrap();
            }
        };
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
