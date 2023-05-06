use crate::raw;

use alsa::pcm::{Access, Format, HwParams, State, PCM};
use alsa::{Direction, ValueOr};

//TODO: Do proper error handling
impl raw::AlsaStream {
    pub fn open_capture_device(source: &str) -> PCM {
        let pcm = PCM::new(source, Direction::Capture, false).unwrap();

        pcm
    }

    pub fn open_hardware_config(pcm: &PCM) -> HwParams {
        let hwp = HwParams::any(&pcm).unwrap();

        hwp
    }

    pub fn set_hardware_config(hwp: &mut HwParams, config: raw::AlsaConfig) {
        hwp.set_channels(config.channel).unwrap();
        hwp.set_rate(config.sample_rate, ValueOr::Nearest).unwrap();
        hwp.set_format(Format::S16LE).unwrap();
        hwp.set_access(Access::RWInterleaved).unwrap();
        hwp.set_period_size_near(config.frames, ValueOr::Nearest)
            .unwrap();
    }

    pub fn attach_config_to_source(hwp: &HwParams, pcm: &PCM) {
        pcm.hw_params(&hwp).unwrap();
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
