use std::cell::RefCell;

use alsa::pcm::Format;
use std::ffi::CString;

#[derive(Debug)]
pub struct AudioData {
    pub stream: &'static mut [f64],
    pub buffer_size: u16,

    pub format: Format,
    pub rate: u32,
    pub channels: u32,
    pub source: CString,
    pub terminate: RefCell<bool>,
}

// implement all alsa sys-binding calls
impl AudioData {}
