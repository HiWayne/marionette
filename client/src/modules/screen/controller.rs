use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::utils::{get_screen_size::get_screen_size, screen_capturer::ScreenCapturer};

use super::service::{get_screen_graphic_service, start_recording_screen, ScreenResponseInfo};

pub async fn get_screen_info_controller() -> (StatusCode, Json<ScreenResponseInfo>) {
    let result = get_screen_size();
    match result {
        Ok(data) => {
            return (
                StatusCode::OK,
                Json(ScreenResponseInfo {
                    status: 1,
                    data: Some(data),
                    message: "",
                }),
            );
        }
        Err(error_message) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ScreenResponseInfo {
                    status: 5,
                    data: None,
                    message: error_message,
                }),
            );
        }
    }
}

pub async fn get_screen_graphic_controller() -> impl IntoResponse {
    let one_second = Duration::from_secs(1);
    let frame_speed = one_second / 60;
    let mut screen_capturer = ScreenCapturer::new(frame_speed);
    let result = get_screen_graphic_service(&mut screen_capturer);
    match result {
        Ok(image_array_buffer) => {
            return (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/png")],
                Body::from(image_array_buffer),
            );
        }
        Err(error_message) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "text/plain")],
                Body::from(error_message),
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartRecordingScreenBody {
    pub unique_number: &'static str,
}

pub async fn start_recording_screen_controller(
    Json(payload): Json<StartRecordingScreenBody>,
) -> (StatusCode, Json<bool>) {
    start_recording_screen(payload.unique_number).await;
    return (StatusCode::OK, Json(true));
}
