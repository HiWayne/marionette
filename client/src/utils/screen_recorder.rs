use std::{
    io::Write,
    process::{Child, Stdio},
};
use tokio::process::Command;

use super::get_screen_size::get_screen_size;

pub struct ScreenRecorder {
    pub width: usize,
    pub height: usize,
    pub frame_rate: u16,
    pub unique_number: &'static str,
    cmd: Option<Command>,
}

pub struct ScreenRecorderParams {
    pub input_width: Option<usize>,
    pub frame_rate: Option<u16>,
    pub unique_number: &'static str,
}

impl ScreenRecorder {
    pub fn new(params: ScreenRecorderParams) -> ScreenRecorder {
        let default_frame_rate = 60;
        match params {
            ScreenRecorderParams {
                input_width,
                frame_rate,
                unique_number,
            } => {
                let screen_info = get_screen_size().unwrap();
                let (width, height) = (screen_info.width, screen_info.height);
                let input_width = input_width.unwrap_or(width);
                let frame_rate = frame_rate.unwrap_or(default_frame_rate);
                if input_width == width {
                    return ScreenRecorder {
                        width,
                        height,
                        frame_rate,
                        unique_number,
                        cmd: None,
                    };
                } else {
                    let ratio: f32 = height as f32 / width as f32;
                    return ScreenRecorder {
                        width: input_width,
                        height: (input_width as f32 * ratio).round() as usize,
                        frame_rate,
                        unique_number,
                        cmd: None,
                    };
                }
            }
        }
    }

    pub async fn start(&mut self) {
        let mut cmd = Command::new("ffmpeg");
        cmd.args([
            "-f",
            "avfoundation",
            "-i",
            "1",
            "-r",
            format!("{}", self.frame_rate).as_str(),
            "-s",
            format!("{}x{}", self.width, self.height).as_str(),
            "-c:v",
            "libx264",
            "-c:a",
            "aac",
            "-f",
            "flv",
            format!("rtmp://127.0.0.1:1935/live/{}", self.unique_number).as_str(),
        ]);
        let output = cmd.output().await.expect("Failed to execute command");
        if output.stderr.len() > 0 {
            // 打印命令的标准错误
            println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        cmd.kill_on_drop(true);
        self.cmd = Some(cmd);
    }

    pub async fn stop(&mut self) -> bool {
        if let Some(cmd) = &mut self.cmd {
            if let Ok(mut child) = cmd.spawn() {
                if let Ok(_) = child.kill().await {
                    return true;
                } else {
                    return false;
                }
            }
        }
        false
    }
}
