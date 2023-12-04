extern crate scrap;
use scrap::{Capturer, Display};
use serde::{Deserialize, Serialize};
use std::io::ErrorKind::WouldBlock;
use tokio::time::{sleep, Duration};

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
    let (w, h) = (capturer.width(), capturer.height());
    print!("w: {}, h: {}", w, h);
    loop {
        // Wait until there's a frame.

        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    sleep(one_frame).await;
                    continue;
                } else {
                    return Err("123");
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

        return Ok(buffer.to_vec());
    }
}
