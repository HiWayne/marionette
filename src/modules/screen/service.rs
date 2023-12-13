extern crate scrap;
use ndarray::Array3;
use scrap::{Capturer, Display};
use serde::{Deserialize, Serialize};
use std::io::ErrorKind::WouldBlock;
use std::thread::sleep;
use tokio::time::Duration;
use video_rs::{Encoder, Time};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenInfo {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenResponseInfo {
    pub status: u8,
    pub data: Option<ScreenInfo>,
    pub message: &'static str,
}

pub fn get_screen_info_service() -> Result<ScreenInfo, &'static str> {
    let display_result = Display::primary();
    if let Ok(display) = display_result {
        return Ok(ScreenInfo {
            width: display.width() as u32,
            height: display.height() as u32,
        });
    }
    Err("capture screen size error")
}

pub async fn get_screen_graphic_service<'a>() -> Result<Vec<u8>, &'static str> {
    let one_second = Duration::from_secs(1);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    loop {
        // Wait until there's a frame.

        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    sleep(one_frame);
                    continue;
                } else {
                    return Err("capturer frame error");
                }
            }
        };

        // Flip the ARGB image into a BGRA image.

        // let mut bitflipped = Vec::with_capacity(w * h * 4);
        // let stride = buffer.len() / h;

        // for y in 0..h {
        //     for x in 0..w {
        //         let i = stride * y + 4 * x;
        //         bitflipped.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i], 255]);
        //     }
        // }
        let vec = buffer.to_vec();
        return Ok(vec);
    }
}

pub fn create_video_service(
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
