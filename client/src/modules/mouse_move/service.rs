use enigo::*;
use serde::{Deserialize, Serialize};

use crate::utils::get_screen_size::get_screen_size;

#[derive(Debug, Serialize, Deserialize)]
pub struct MousePosition {
    pub x: f32,
    pub y: f32,
}

pub async fn mouse_move_service(mouse_position: MousePosition) -> bool {
    let screen_info_result = get_screen_size();
    match screen_info_result {
        Ok(screen_info) => {
            let mut enigo = Enigo::new();
            enigo.mouse_move_to(
                (mouse_position.x * screen_info.width as f32).round() as i32,
                (mouse_position.y * screen_info.height as f32).round() as i32,
            );
            return true;
        }
        Err(_) => false,
    }
}
