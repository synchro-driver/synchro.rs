use std::os::raw::c_char;

#[link(name = "audioinput", kind = "static")]
extern "C" {
    fn audio_fetch() -> audio_data;
}

#[derive(Debug)]
pub struct audio_data {
    stream: f64,
    buffer_size: i32,

    format: i32,
    rate: u32,
    channels: u32,
    source: c_char,
    im: i32,
    error_messaage: [char; 1024],
    sample_counter: i32,
}

impl audio_frame {
    pub fn fetch_audio() -> audio_data{
        let audio_frame: audio_data;
        unsafe {
            audio_frame = audio_fetch();
        }
        println!(audio_frame);

        audio_frame
    }
}