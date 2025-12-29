pub fn draw_rect(
    buffer: &mut [u8],
    info: bootloader_api::info::FrameBufferInfo,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: (u8, u8, u8), // (R, G, B)
) {
    let bytes_per_pixel = info.bytes_per_pixel;
    let stride = info.stride;

    let (r, g, b) = color;

    for py in y..(y + height) {
        for px in x..(x + width) {
            let idx =
                py * stride * bytes_per_pixel +
                px * bytes_per_pixel;

            // BGR format
            buffer[idx]     = b;
            buffer[idx + 1] = g;
            buffer[idx + 2] = r;
        }
    }
}
