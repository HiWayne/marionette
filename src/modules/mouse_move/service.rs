use crate::modules::screen::service::get_screen_info_service;
use enigo::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MousePosition {
    pub x: f32,
    pub y: f32,
}

pub async fn mouse_move_service(mouse_position: MousePosition) -> bool {
    let screen_info = get_screen_info_service();
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(
        (mouse_position.x * screen_info.width as f32).round() as i32,
        (mouse_position.y * screen_info.height as f32).round() as i32,
    );
    return true;
}
