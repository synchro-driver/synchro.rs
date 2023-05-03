use std::os::raw::{c_char, c_float, c_int, c_uint};

#[link(name = "audioinput")]
extern "C" {
    fn audio_fetch() -> audio_data;
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct audio_data {
    stream: *mut c_float,
    buffer_size: c_int,

   format: c_int,
    rate: c_uint,
    channels: c_uint,
    source: c_char,
    im: c_int,
    error_messaage: [c_char; 1024],
    sample_counter: c_int,
}

impl audio_data {
    pub fn fetch_audio() -> Result<audio_data, ()>{
        let audio_frame: audio_data = unsafe { audio_fetch() };
        println!("{:?}", audio_frame);

        match audio_frame.buffer_size {
            -1 => Err(()),
            0 => Err(()),
            _ => Ok(audio_frame)
        }
    }
}