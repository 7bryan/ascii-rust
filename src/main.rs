mod audio;
mod clock;
mod frame;
mod render;

use std::io::{self, Read};
use std::process::{Command, Stdio};

use crate::audio::start_audio_playback;

// helper function to get the source video width and height
fn get_video_dimensions(path: &str) -> (usize, usize) {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height",
            "-of",
            "csv=s=x:p=0",
            path,
        ])
        .output()
        .expect("Failed to excute ffprobe");

    let result = String::from_utf8(output.stdout).expect("Invalid UTF-8 from ffprobe");

    // parses string into numbers (1920x1080)
    let parts: Vec<&str> = result.trim().split('x').collect();

    let source_width = parts[0].parse::<usize>().unwrap_or(16);
    let sourcec_height = parts[1].parse::<usize>().unwrap_or(9);

    (source_width, sourcec_height)
}

fn main() {
    let video_path = "assets/bad_apple.mp4";
    let audio_path = audio::extract_audio(&video_path);

    let (source_width, source_height) = get_video_dimensions(video_path);

    let (term_cols, term_rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let width = term_cols as usize;
    let height = (width as f32 * (source_height as f32 / source_width as f32) * 0.5) as usize;
    let height = height.min(term_rows as usize); // don't exceed terminal height either

    // Flip this to false to fall back to the faster grayscale renderer.
    let use_color = true;

    let mut child = Command::new("ffmpeg")
        .args([
            "-i",
            video_path,
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgb24",
            "-s",
            &format!("{}x{}", width, height),
            "-v",
            "quiet",
            "-",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn ffmpeg. is it on PATH?");

    let mut stdout = child.stdout.take().expect("no stdout handle");
    let mut term = io::stdout();

    let frame_size = width * height * 3; // 3 bytes per pixel (RGB)
    let mut buffer = vec![0u8; frame_size];

    let mut frame_count = 0;

    let fps = clock::get_video_fps(video_path).unwrap_or(30.0);

    // start audio playback concurrently right before the video frame loop starts
    let (_stream, _sink) = start_audio_playback(&audio_path);
    let timer = clock::Clock::new(fps);

    render::init(&mut term);

    loop {
        let target = timer.target_frame();

        if frame_count > target {
            std::thread::sleep(std::time::Duration::from_millis(5));
            continue;
        }

        match stdout.read_exact(&mut buffer) {
            Ok(_) => {
                frame_count += 1;

                if frame_count >= target {
                    let ascii = if use_color {
                        frame::frame_to_ascii_color(&buffer, width, height)
                    } else {
                        frame::frame_to_ascii(&buffer, width, height)
                    };
                    render::draw_frame(&mut term, &ascii);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(e) => {
                eprintln!("Error reading frame: {}", e);
                break;
            }
        }
    }

    render::cleanup(&mut term);

    println!("Total frames read: {}", frame_count);

    child.wait().expect("ffmpeg didn't exit cleanly");
}
