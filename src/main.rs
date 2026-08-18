use std::io::Read;
use std::process::{Command, Stdio};

fn main() {
    let video_path = "assets/bad_apple.mp4";
    let width = 80;
    let height = 45;

    let mut child = Command::new("ffmpeg")
        .args([
            // decode the video using ffmpeg
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

    let frame_size = width * height * 3; // 3 bytes per pixel (RGB)
    let mut buffer = vec![0u8; frame_size];

    let mut frame_count = 0;

    loop {
        // fills the buffer completely or returns an error
        match stdout.read_exact(&mut buffer) {
            Ok(_) => {
                frame_count += 1;
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                // ffmpeg ran out of video frames safely
                break;
            }
            Err(e) => {
                // handles actual reading errors
                eprintln!("Error reading frame: {}", e);
                break;
            }
        }
    }

    println!("Total frames read: {}", frame_count);

    child.wait().expect("ffmpeg didn't exit cleanly");
}
