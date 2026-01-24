pub fn asteroid_shape_eclipse_radii(seed: u64, scale: f32) -> (f32, f32) {
    let rx = scale * (0.7 + (seed >> 56) as f32 * (0.3 / 255.0));
    let ry = scale * (0.7 + ((seed >> 48) & 0xFF) as f32 * (0.3 / 255.0));
    (rx, ry)
}
