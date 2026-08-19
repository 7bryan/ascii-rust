use std::process::Command;
use std::str;
use std::time::Instant;

pub struct Clock {
    start: Instant,
    fps: f64,
}

impl Clock {
    pub fn new(fps: f64) -> Self {
        Self {
            start: Instant::now(),
            fps,
        }
    }

    /// Returns which frame index SHOULD be displaying right now.
    pub fn target_frame(&self) -> usize {
        let elapsed = self.start.elapsed().as_secs_f64();
        (elapsed * self.fps) as usize
    }
}

pub fn get_video_fps(path: &str) -> Result<f64, String> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=r_frame_rate",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path,
        ])
        .output()
        .map_err(|e| format!("Failed to execute ffrope: {}", e))?;

    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown error");

        return Err(format!("ffprope error: {}", stderr));
    }

    // convert output bytes to a string slice and clean up whitespace
    let stdout = str::from_utf8(&output.stdout)
        .map_err(|e| format!("Invalid UTF-8 output: {}", e))?
        .trim();

    if let Some((num_str, den_str)) = stdout.split_once("/") {
        let numerator: f64 = num_str.parse().map_err(|_| "Invalid numerator")?;
        let denominator: f64 = den_str.parse().map_err(|_| "Invalid denominator")?;

        if denominator == 0.0 {
            return Err("Denominator cannot be zero".to_string());
        }

        Ok(numerator / denominator)
    } else {
        stdout
            .parse::<f64>()
            .map_err(|_| format!("Unexpected out put format: {}", stdout))
    }

    // TODO: run ffprobe with -show_entries stream=r_frame_rate
    // TODO: parse output like "30/1" or "30000/1001"
    // TODO: split on '/', parse both sides as f64, divide numerator/denominator
}
