extern crate scrap;
use ndarray::Array3;
use scrap::{Capturer, Display};
use serde::{Deserialize, Serialize};
use std::io::ErrorKind::WouldBlock;
use std::thread::sleep;
use tokio::time::Duration;
use video_rs::{Encoder, Time};

use crate::utils::screen_capturer::{self, ScreenCapturer};

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

pub fn add_frame_to_video_service(
    encoder: &mut Encoder,
    position: Time,
    duration: &Time,
    image_buffer: &Vec<u8>,
    width: usize,
    height: usize,
) -> Time {
    // This creates a frame with height 1080, width 1920 and three
    // channels. The RGB values for each pixel are equal, and
    // determined by the `rgb` we chose above.
    let frame = Array3::from_shape_fn((height, width, 3), |(y, x, c)| {
        let color_result = image_buffer.get(x * 3 + y * width * 3 + c);
        if let Some(color) = color_result {
            return *color;
        } else {
            println!("invalid color");
            return 0;
        }
    });

    encoder
        .encode(&frame, &position)
        .expect("failed to encode frame");

    // Update the current position and add the inter-frame
    // duration to it.
    return position.aligned_with(duration).add();
}
