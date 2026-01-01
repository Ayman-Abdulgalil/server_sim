pub fn angle_difference(target: f32, current: f32) -> f32 {
    let mut diff = target - current;

    while diff > std::f32::consts::PI {
        diff -= 2.0 * std::f32::consts::PI;
    }
    while diff < -std::f32::consts::PI {
        diff += 2.0 * std::f32::consts::PI;
    }

    diff
}

pub fn exp_smooth_factor(sharpness: f32, dt: f32) -> f32 {
    1.0 - (-sharpness * dt).exp()
}
