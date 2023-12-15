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

pub mod utils {
    pub mod get_screen_size;
    pub mod screen_capturer;
    pub mod video_generator;
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

use modules::screen::service::{add_frame_to_video_service, get_screen_graphic_service};
use tokio::time::Duration;
use utils::{screen_capturer::ScreenCapturer, video_generator::VideoGenerator};

#[tokio::main]
async fn main() {
    let mut video_generator = VideoGenerator::new("rainbow.mp4");
    let one_second = Duration::from_secs(1);
    let frame_speed = one_second / 24;
    let mut screen_capturer = ScreenCapturer::new(frame_speed);
    let mut frame_count = 0;
    loop {
        if frame_count > 72 {
            break;
        }
        let image_buffer = get_screen_graphic_service(&mut screen_capturer).unwrap();
        video_generator.position = add_frame_to_video_service(
            &mut video_generator.encoder,
            video_generator.position,
            &video_generator.duration,
            &image_buffer,
            video_generator.width as usize,
            video_generator.height as usize,
        );
        frame_count += 1;
    }
    video_generator
        .encoder
        .finish()
        .expect("failed to finish encoder");
}
