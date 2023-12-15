use std::time::Duration;

use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};

use crate::utils::{get_screen_size::get_screen_size, screen_capturer::ScreenCapturer};

use super::service::{get_screen_graphic_service, ScreenResponseInfo};

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
            println!("444, ok, {}", image_array_buffer.len());
            return (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/png")],
                Body::from(image_array_buffer),
            );
        }
        Err(error_message) => {
            println!("444, err");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "text/plain")],
                Body::from(error_message),
            );
        }
    }
}
