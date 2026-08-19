use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::process::Command;

pub fn start_audio_playback(audio_path: &str) -> (OutputStream, Sink) {
    // getting output stream and stream handle
    // let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let (stream, stream_handle) = OutputStream::try_default().expect("Failed to get audio device");

    // creating a sink attached to the stream handle
    // let sink = Sink::try_new(&stream_handle).unwrap();
    let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");

    // open and load the audio file
    let file =
        File::open(audio_path).unwrap_or_else(|_| panic!("Audio file not found at {}", audio_path));

    // let source = Decoder::new(BufReader::new(file)).unwrap();
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio file");

    // play the sound
    sink.append(source);

    (stream, sink)
    // blocking the thread so the program doesn't exit instantly
    // sink.sleep_until_end();
}

pub fn extract_audio(path: &str) -> String {
    let output_path = path.replace(".mp4", ".wav");

    let _output = Command::new("ffmpeg")
        .args([
            "-y", // overwrite the existing file is exist
            "-i",
            path,
            "-vn",
            "-acodec",
            "pcm_s16le",
            "-ar",
            "44100",
            &output_path,
        ])
        .output()
        .expect("extracting failed");

    if !_output.status.success() {
        eprintln!(
            "FFmpeg Error:\n{}",
            String::from_utf8_lossy(&_output.stderr)
        );
        panic!("Failed to extract audio from video file");
    }

    output_path
}
