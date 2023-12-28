use axum::{
    routing::{get, post},
    Router,
};

pub mod modules {
    pub mod app {
        pub mod controller;
        pub mod service;
    }
    pub mod mouse_move {
        pub mod controller;
        pub mod service;
    }
    pub mod screen {
        pub mod controller;
        pub mod service;
    }
}

pub mod utils {
    pub mod get_screen_size;
    pub mod screen_capturer;
    pub mod screen_recorder;
}

use modules::{app, mouse_move, screen::{self, service::start_recording_screen}};
use utils::screen_recorder::{ScreenRecorder, ScreenRecorderParams};

#[tokio::main]
async fn main() {
    start_recording_screen("123").await;
    // let app = Router::new()
    //     .route("/", get(app::controller::app_controller))
    //     .route(
    //         "/api/screen/info",
    //         get(screen::controller::get_screen_info_controller),
    //     )
    //     .route(
    //         "/api/screen/capture",
    //         get(screen::controller::get_screen_graphic_controller),
    //     )
    //     .route(
    //         "/api/mouse/move",
    //         post(mouse_move::controller::mouse_move_controller),
    //     )
    //     .route(
    //         "/api/start/recording/screen",
    //         post(screen::controller::start_recording_screen_controller),
    //     );

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:10100")
    //     .await
    //     .unwrap();
    // axum::serve(listener, app).await.unwrap();
}
