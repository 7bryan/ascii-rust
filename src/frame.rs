const GRADIENT: &[u8] = b" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

/// Convert one raw RGB frame buffer into a printable ASCII string.
pub fn frame_to_ascii(buffer: &[u8], width: usize, height: usize) -> String {
    let mut output = String::with_capacity(width * height + height); // +height for newlines

    for row in 0..height {
        for col in 0..width {
            let pixel_idx = (row * width + col) * 3; // each pixel have R, G, B value

            let r = buffer[pixel_idx] as f32;
            let g = buffer[pixel_idx + 1] as f32;
            let b = buffer[pixel_idx + 2] as f32;

            let luminance: f32 = 0.299 * r + 0.587 * g + 0.114 * b;

            let idx = (luminance / 255.0 * (GRADIENT.len() - 1) as f32) as usize;

            output.push(GRADIENT[idx] as char);
        }
        output.push('\n');
    }

    output
}
