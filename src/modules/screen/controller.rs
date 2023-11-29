use axum::{http::StatusCode, Json};

use super::service::{get_screen_info_service, ScreenInfo};

pub async fn get_screen_info_controller() -> (StatusCode, Json<ScreenInfo>) {
    let result = get_screen_info_service();
    return (StatusCode::OK, Json(result));
}
