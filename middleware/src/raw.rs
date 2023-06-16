use std::ffi::CString;
use std::os::unix::thread;

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
}

pub struct AlsaStream {
    // Default size of frame is 1024
    pub stream: [i16; 1024],
    pub pcm: PCM,
}

impl AlsaStream {
    pub fn new(soure: &str, is_input: bool) -> AlsaStream {
        if is_input {
            AlsaStream {
                stream: Self::open_stream_buffer(),
                pcm: Self::open_capture_device(CString::new(soure).unwrap()),
            }
        } else {
            AlsaStream {
                stream: Self::open_stream_buffer(),
                pcm: Self::open_playback_device(CString::new(soure).unwrap()),
            }
        }
    }

    pub fn open_stream_buffer() -> [i16; 1024] {
        let buffer = [0i16; 1024];

        buffer
    }

    pub fn open_capture_device(source: CString) -> PCM {
        let pcm = PCM::new(source.to_str().unwrap(), Direction::Capture, false).unwrap();

        pcm
    }

    pub fn open_playback_device(source: CString) -> PCM {
        let pcm = PCM::new(source.to_str().unwrap(), Direction::Playback, true).unwrap();

        pcm
    }

    pub fn open_hardware_config<'a>(stream: &'a AlsaStream) -> HwParams<'a> {
        let hw = HwParams::any(&stream.pcm).unwrap();

        hw
    }

    pub fn set_hardware_config(hw: &HwParams, config: &mut AlsaConfig) {
        hw.set_channels(config.channel).unwrap();
        hw.set_rate(config.sample_rate, ValueOr::Nearest).unwrap();
        hw.set_format(Format::S16LE).unwrap();
        hw.set_access(Access::RWInterleaved).unwrap();
        hw.set_period_size_near(config.frame_size, ValueOr::Nearest)
            .unwrap();
    }

    pub fn attach_config_to_capture(&self, hw: &HwParams) {
        self.pcm.hw_params(&hw).unwrap();
    }

    pub fn read_from_io(&mut self) {
        let io_handler = self.pcm.io_i16().unwrap();

        match io_handler.readi(&mut self.stream) {
            Ok(bytes) => println!("{} bytes read", bytes),
            Err(err) => {
                eprintln!("Overflow Occured {}", err);
                self.pcm.prepare().unwrap();
            }
        };
    }

    pub fn write_to_io(&mut self, stream_arr: &[i16]) {
        let io_handler = self.pcm.io_i16().unwrap();

        // println!("{:?}", stream_arr);

        // for _ in 0..1 * 44100 {
        match io_handler.writei(&stream_arr) {
            Ok(bytes) => println!("{} bytes flushed to IO", bytes),
            Err(err) => eprintln!("Flush Failed! {}", err),
        }
        // }
    }

    pub fn close_capture_device(pcm: PCM) {
        pcm.drop().unwrap();
    }

    pub fn get_transfer_size(&self) -> (u64, u64) {
        self.pcm.get_params().unwrap()
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
