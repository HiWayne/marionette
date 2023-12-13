// use axum::{
//     routing::{get, post},
//     Router,
// };

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

// use modules::{app, mouse_move, screen};

// #[tokio::main]
// async fn main() {
//     // our router
//     let app = Router::new()
//         .route("/", get(app::controller::app_controller))
//         .route(
//             "/api/screen/info",
//             get(screen::controller::get_screen_info_controller),
//         )
//         .route(
//             "/api/screen/graphic",
//             get(screen::controller::get_screen_graphic_controller),
//         )
//         .route(
//             "/api/mouse/move",
//             post(mouse_move::controller::mouse_move_controller),
//         );

//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

use std::path::PathBuf;

use modules::screen::service::{
    create_video_service, get_screen_graphic_service, get_screen_info_service,
};
use video_rs::{Encoder, EncoderSettings, Locator, Time};

#[tokio::main]
async fn main() {
    let screen_info = get_screen_info_service().unwrap();
    video_rs::init().unwrap();
    let destination: Locator = PathBuf::from("rainbow.mp4").into();
    let settings = EncoderSettings::for_h264_yuv420p(
        screen_info.width as usize,
        screen_info.height as usize,
        false,
    );
    let mut encoder = Encoder::new(&destination, settings).expect("failed to create encoder");
    let duration: Time = Time::from_nth_of_a_second(60);
    let mut position = Time::zero();
    let mut frame_count = 0;
    loop {
        if frame_count > 24 {
            break;
        }
        let image_buffer = get_screen_graphic_service().await.unwrap();
        position = create_video_service(
            &mut encoder,
            position,
            &duration,
            &image_buffer,
            screen_info.width as usize,
            screen_info.height as usize,
        );
        frame_count += 1;
    }
    encoder.finish().expect("failed to finish encoder");
}
