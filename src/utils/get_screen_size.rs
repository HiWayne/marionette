use scrap::Display;

use crate::modules::screen::service::ScreenInfo;

pub fn get_screen_size() -> Result<ScreenInfo, &'static str> {
    let display_result = Display::primary();
    if let Ok(display) = display_result {
        return Ok(ScreenInfo {
            width: display.width(),
            height: display.height(),
        });
    }
    Err("get screen size error")
}
