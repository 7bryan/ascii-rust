const GRADIENT: &[u8] = b" .:-=+*#%@";

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

            // TODO: figure out the index into `buffer` for pixel (col, row).
            // Hint: buffer is flat RGB data, row-major order.
            // Each pixel takes 3 bytes. Each row takes width*3 bytes.
            // pixel_index = (row * width + col) * 3
            // r = buffer[pixel_index], g = buffer[pixel_index+1], b = buffer[pixel_index+2]

            // TODO: compute luminance using the formula above (as f32 math, then cast to u8 or usize)

            // TODO: map luminance to a GRADIENT index and push GRADIENT[index] as char into `output`

            output.push(GRADIENT[idx] as char);
        }
        output.push('\n');
    }

    output
}
