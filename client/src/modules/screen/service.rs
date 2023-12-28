extern crate scrap;
use serde::{Deserialize, Serialize};
use std::io::ErrorKind::WouldBlock;
use std::thread::sleep;

use crate::utils::screen_capturer::ScreenCapturer;
use crate::utils::screen_recorder::{ScreenRecorder, ScreenRecorderParams};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenInfo {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenResponseInfo {
    pub status: u8,
    pub data: Option<ScreenInfo>,
    pub message: &'static str,
}

pub fn get_screen_graphic_service<'a>(
    screen_capturer: &mut ScreenCapturer,
) -> Result<Vec<u8>, &'static str> {
    let (capturer, frame_speed, width, height) = (
        &mut screen_capturer.capturer,
        screen_capturer.frame_speed,
        screen_capturer.width,
        screen_capturer.height,
    );
    loop {
        // Wait until there's a frame.
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    sleep(frame_speed);
                    continue;
                } else {
                    return Err("capturer frame error");
                }
            }
        };

        // Flip the ARGB image into a RGB image.

        let mut bitflipped = Vec::with_capacity(width * height * 3);
        let stride = buffer.len() / height;

        for y in 0..height {
            for x in 0..width {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i]]);
            }
        }
        return Ok(bitflipped);
    }
}

pub async fn start_recording_screen(unique_number: &'static str) {
    let mut screen_recorder = ScreenRecorder::new(ScreenRecorderParams {
        input_width: None,
        frame_rate: None,
        unique_number
    });
    screen_recorder.start().await;
}