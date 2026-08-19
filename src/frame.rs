use std::fmt::Write as FmtWrite;

const GRADIENT: &[u8] = b" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

/// Convert one raw RGB frame buffer into a printable ASCII string.
pub fn frame_to_ascii(buffer: &[u8], width: usize, height: usize) -> String {
    let mut output = String::with_capacity(width * height + height);

    for row in 0..height {
        for col in 0..width {
            let pixel_idx = (row * width + col) * 3; // each pixel has R, G, B

            let r = buffer[pixel_idx] as f32;
            let g = buffer[pixel_idx + 1] as f32;
            let b = buffer[pixel_idx + 2] as f32;

            let luminance: f32 = 0.299 * r + 0.587 * g + 0.114 * b;

            let idx = (luminance / 255.0 * (GRADIENT.len() - 1) as f32) as usize;

            output.push(GRADIENT[idx] as char);
        }
        // Only push a newline BETWEEN rows, not after the last one.
        if row < height - 1 {
            output.push('\n');
        }
    }

    output
}

/// wraps each character in a 24-bit ANSI
pub fn frame_to_ascii_color(buffer: &[u8], width: usize, height: usize) -> String {
    // Rough capacity guess: ~20 bytes per colored char + newlines.
    let mut output = String::with_capacity(width * height * 20 + height);

    for row in 0..height {
        for col in 0..width {
            let pixel_idx = (row * width + col) * 3;

            let r = buffer[pixel_idx];
            let g = buffer[pixel_idx + 1];
            let b = buffer[pixel_idx + 2];

            let luminance: f32 = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
            let idx = (luminance / 255.0 * (GRADIENT.len() - 1) as f32) as usize;
            let ch = GRADIENT[idx] as char;

            // write! into the String directly -- no per-pixel format!()
            // allocation, which would otherwise happen thousands of
            // times per frame.
            let _ = write!(output, "\x1b[38;2;{};{};{}m{}", r, g, b, ch);
        }
        // Reset color at the end of every row. Without this, some
        // terminals can bleed the last color into blank space or
        // the next row in unpredictable ways.
        output.push_str("\x1b[0m");

        if row < height - 1 {
            output.push('\n');
        }
    }

    output
}
