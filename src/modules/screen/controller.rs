use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};

use super::service::{get_screen_graphic_service, get_screen_info_service, ScreenResponseInfo};

pub async fn get_screen_info_controller() -> (StatusCode, Json<ScreenResponseInfo>) {
    let result = get_screen_info_service();
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
    let result = get_screen_graphic_service().await;
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
