use std::path::PathBuf;

use video_rs::{Encoder, EncoderSettings, Locator, Time};

use super::get_screen_size::get_screen_size;

pub struct VideoGenerator {
    pub encoder: Encoder,
    pub position: Time,
    pub duration: Time,
    pub width: usize,
    pub height: usize,
}

impl VideoGenerator {
    pub fn new(file_path: &'static str) -> VideoGenerator {
        let screen_info = get_screen_size().unwrap();
        let (width, height) = (screen_info.width, screen_info.height);
        video_rs::init().unwrap();
        let destination: Locator = PathBuf::from(file_path).into();
        let settings = EncoderSettings::for_h264_yuv420p(
            screen_info.width as usize,
            screen_info.height as usize,
            false,
        );
        let mut encoder = Encoder::new(&destination, settings).expect("failed to create encoder");
        let duration: Time = Time::from_nth_of_a_second(24);
        let mut position = Time::zero();
        VideoGenerator {
            encoder,
            position,
            duration,
            width,
            height,
        }
    }
}
