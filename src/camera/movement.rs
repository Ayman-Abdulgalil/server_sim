use bevy::prelude::*;

use super::consts;
use super::math::{angle_difference, exp_smooth_factor};
use super::settings::CamSettings;

pub fn apply_speed_scroll(scroll_y: f32, cam_sets: &mut CamSettings) {
    if scroll_y == 0.0 {
        return;
    }

    let k = (scroll_y * cam_sets.sensitivity * consts::SPEED_SCROLL_RATE).exp();
    cam_sets.speed = (cam_sets.speed * k).clamp(cam_sets.speed_range.start, cam_sets.speed_range.end);
}

pub fn update_look_targets(delta: Vec2, cam_sets: &mut CamSettings) {
    let rot_per_pixel = consts::ROT_PER_PIXEL * cam_sets.rotation_scale();

    cam_sets.target_yaw -= delta.x * rot_per_pixel;
    cam_sets.target_pitch -= delta.y * rot_per_pixel;

    cam_sets.target_pitch = cam_sets
        .target_pitch
        .clamp(-cam_sets.pitch_limit, cam_sets.pitch_limit);
}

pub fn apply_smoothed_rotation(dt: f32, cam_sets: &mut CamSettings, cam: &mut Transform) {
    let (current_yaw, current_pitch, _) = cam.rotation.to_euler(EulerRot::YXZ);
    let a = exp_smooth_factor(cam_sets.rotation_smoothing, dt);

    let yaw_diff = angle_difference(cam_sets.target_yaw, current_yaw);
    let yaw = current_yaw + yaw_diff * a;
    let pitch = current_pitch + (cam_sets.target_pitch - current_pitch) * a;

    // This yaw/pitch convention commonly uses EulerRot::YXZ. [web:33]
    cam.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

pub fn compute_target_velocity(
    keyboard_input: &ButtonInput<KeyCode>,
    cam: &Transform,
    speed: f32,
) -> Vec3 {
    let mut v = Vec3::ZERO;

    let forward = cam.forward();
    let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right = *cam.right();

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        v += forward_flat;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        v -= forward_flat;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        v -= right;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        v += right;
    }

    if keyboard_input.pressed(KeyCode::Space)
        || keyboard_input.pressed(KeyCode::ShiftRight)
        || keyboard_input.pressed(KeyCode::KeyE)
    {
        v += Vec3::Y * consts::VERTICAL_MOVE_SCALE;
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft)
        || keyboard_input.pressed(KeyCode::ControlRight)
        || keyboard_input.pressed(KeyCode::KeyQ)
    {
        v -= Vec3::Y * consts::VERTICAL_MOVE_SCALE;
    }

    v.normalize_or_zero() * speed
}

pub fn update_velocity(dt: f32, target_velocity: Vec3, cam_sets: &mut CamSettings) {
    let accel_a = exp_smooth_factor(cam_sets.acceleration, dt);
    let fric_a = exp_smooth_factor(cam_sets.friction, dt);

    if target_velocity.length() > 0.01 {
        cam_sets.current_velocity = cam_sets.current_velocity.lerp(target_velocity, accel_a);
    } else {
        cam_sets.current_velocity = cam_sets.current_velocity.lerp(Vec3::ZERO, fric_a);
    }
}

pub fn apply_friction_and_coast(dt: f32, cam_sets: &mut CamSettings, cam: &mut Transform) {
    let fric_a = exp_smooth_factor(cam_sets.friction, dt);

    if cam_sets.current_velocity.length() > 0.001 {
        cam_sets.current_velocity = cam_sets.current_velocity.lerp(Vec3::ZERO, fric_a);
        cam.translation += cam_sets.current_velocity * dt;
        clamp_translation(cam, cam_sets);
    } else {
        cam_sets.current_velocity = Vec3::ZERO;
    }
}

pub fn clamp_translation(cam: &mut Transform, cam_sets: &CamSettings) {
    let (min_x, max_x, min_z, max_z) = cam_sets.bounds_with_margin();

    cam.translation.x = cam.translation.x.clamp(min_x, max_x);
    cam.translation.y = cam.translation.y.clamp(cam_sets.y_limit.start, cam_sets.y_limit.end);
    cam.translation.z = cam.translation.z.clamp(min_z, max_z);
}
