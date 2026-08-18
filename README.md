# ascii-rust

A terminal video to ASCII converter
Built using rust and FFmpeg

## Project Structure

```
ascii-rust/
├── assets/
│   └── bad_apple.mp4   # test video (bad apple)
├── src/
│   ├── audio.rs        # rodio setup (audio playback library)
│   ├── clock.rs        # master clock / frame timing logic
│   ├── ffmpeg.rs       # spawn FFmpeg processes (video pipe and audio extract)
│   ├── main.rs         # entry point, arg parsing, orchestration
│   └── render.rs       # crossterm output, cursor positioning
├── Cargo.toml
└── .gitignore
```
