use captrs::Capturer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenInfo {
    pub width: u32,
    pub height: u32,
}

pub fn get_screen_info_service() -> ScreenInfo {
    let capturer = Capturer::new(0).unwrap();
    let (w, h) = capturer.geometry();
    return ScreenInfo {
        width: w,
        height: h,
    };
}
