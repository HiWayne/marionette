use std::time::Duration;

use scrap::{Capturer, Display};

pub struct ScreenCapturer {
    pub capturer: Capturer,
    pub width: usize,
    pub height: usize,
    pub frame_speed: Duration,
}

impl ScreenCapturer {
    pub fn new(frame_speed: Duration) -> ScreenCapturer {
        let display = Display::primary().expect("Couldn't find primary display.");
        let (width, height) = (display.width(), display.height());
        let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
        ScreenCapturer {
            width,
            height,
            capturer,
            frame_speed,
        }
    }
}
