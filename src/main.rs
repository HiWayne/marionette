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

use modules::{app, mouse_move, screen};

#[tokio::main]
async fn main() {
    // our router
    let app = Router::new()
        .route("/", get(app::controller::app_controller))
        .route(
            "/api/screen/info",
            get(screen::controller::get_screen_info_controller),
        )
        .route(
            "/api/screen/graphic",
            get(screen::controller::get_screen_graphic_controller),
        )
        .route(
            "/api/mouse/move",
            post(mouse_move::controller::mouse_move_controller),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
