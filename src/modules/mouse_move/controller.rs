use super::service::{mouse_move_service, MousePosition};
use axum::{http::StatusCode, Json};

pub async fn mouse_move_controller(Json(payload): Json<MousePosition>) -> (StatusCode, Json<bool>) {
    let result = mouse_move_service(payload).await;
    return (StatusCode::OK, Json(result));
}
