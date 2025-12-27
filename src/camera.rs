use bevy::{
    input::{
        ButtonInput,
        mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    },
    prelude::*,
    window::{CursorOptions, PrimaryWindow},
};
use std::f32::consts::FRAC_PI_2;
use std::ops::Range;

/// Camera modes
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum CamState {
    #[default]
    Fixed,
    Free,
}

/// Camera configuration and runtime state
#[derive(Resource, Default)]
pub struct CamSettings {
    /// Current camera mode (Fixed or Free)
    pub cam_state: CamState,
    /// Initial camera position (used during setup)
    pub cam_position: Vec3,
    /// Point the camera looks at initially (used during setup)
    pub cam_target: Vec3,

    // === Boundary Settings ===
    /// Extra margin around board boundaries where camera can move
    nav_margin: f32,
    /// X-axis movement limits (min..max)
    x_limit: Range<f32>,
    /// Y-axis (height) movement limits (min..max)
    y_limit: Range<f32>,
    /// Z-axis movement limits (min..max)
    z_limit: Range<f32>,

    // === Movement Settings ===
    /// Current movement speed (units per second)
    speed: f32,
    /// Allowed speed range (min..max)
    speed_range: Range<f32>,
    /// Mouse wheel sensitivity for speed adjustment
    sensitivity: f32,
    /// How quickly velocity reaches target (higher = snappier)
    acceleration: f32,
    /// How quickly velocity decays when no input (higher = stops faster)
    friction: f32,
    /// Current smoothed velocity vector
    current_velocity: Vec3,

    // === Rotation Settings ===
    /// Maximum pitch angle (prevents looking straight up/down)
    pitch_limit: f32,
    /// Target yaw angle (horizontal rotation) that camera smoothly moves toward
    target_yaw: f32,
    /// Target pitch angle (vertical rotation) that camera smoothly moves toward
    target_pitch: f32,
    /// Rotation interpolation speed (higher = snappier)
    rotation_smoothing: f32,

    // === UI State ===
    /// Cursor position before entering Free mode (restored on exit)
    last_cursor_pos: Option<Vec2>,
}

/// Spawns the camera and initializes all settings
pub fn setup_camera(mut commands: Commands, mut cam_sets: ResMut<CamSettings>) {
    // Create initial camera transform looking at target
    let initial_transform =
        Transform::from_translation(cam_sets.cam_position).looking_at(cam_sets.cam_target, Vec3::Y);

    // Initialize rotation targets to match initial orientation
    // This prevents unwanted camera spinning on first frame
    let (yaw, pitch, _) = initial_transform.rotation.to_euler(EulerRot::YXZ);
    cam_sets.target_yaw = yaw;
    cam_sets.target_pitch = pitch;

    commands.spawn((Camera3d::default(), initial_transform));

    // === Boundary Configuration ===
    cam_sets.nav_margin = 8.0; // Extra space around board edges
    cam_sets.x_limit = -cam_sets.nav_margin..32.0;
    cam_sets.y_limit = 2.0..50.0; // Min height prevents clipping, max prevents going too high
    cam_sets.z_limit = -cam_sets.nav_margin..32.0;

    // === Movement Configuration ===
    cam_sets.speed = 10.0; // Default movement speed
    cam_sets.speed_range = 1.0..32.0; // Speed can be adjusted between these values
    cam_sets.sensitivity = 4.0; // Mouse wheel speed adjustment sensitivity
    cam_sets.acceleration = 6.0; // How fast camera reaches target speed
    cam_sets.friction = 6.0; // How fast camera stops when no input
    cam_sets.current_velocity = Vec3::ZERO;

    // === Rotation Configuration ===
    cam_sets.pitch_limit = FRAC_PI_2 - 0.01; // ~89.4 degrees, prevents gimbal lock
    cam_sets.rotation_smoothing = 10.0; // Rotation interpolation speed
}

/// Calculates shortest angular distance between two angles.
/// Handles wrapping at ±π to prevent the "long way around" rotation.
///
/// # Example
/// 
/// Without wrapping: 3.1 -> -3.1 = -6.2 (spins almost full circle)
/// With wrapping:    3.1 -> -3.1 = 0.08 (tiny rotation)
/// 
fn angle_difference(target: f32, current: f32) -> f32 {
    let mut diff = target - current;
    
    // Normalize to [-π, π] range
    while diff > std::f32::consts::PI {
        diff -= 2.0 * std::f32::consts::PI;
    }
    while diff < -std::f32::consts::PI {
        diff += 2.0 * std::f32::consts::PI;
    }
    
    diff
}

/// Main camera control system - handles input and applies smooth movement/rotation
pub fn cam_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_wheel: Res<AccumulatedMouseScroll>,
    mut cam_sets: ResMut<CamSettings>,
    mut cam: Single<&mut Transform, With<Camera>>,
    mut cursor_options: Single<&mut CursorOptions>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    // === Mode Toggle (Tab key) ===
    if keyboard_input.just_pressed(KeyCode::Tab) {
        cam_sets.cam_state = match cam_sets.cam_state {
            CamState::Fixed => {
                // Entering Free mode: hide cursor and save its position
                cam_sets.last_cursor_pos = window.cursor_position();
                cursor_options.visible = false;
                CamState::Free
            }
            CamState::Free => {
                // Exiting Free mode: restore cursor position and visibility
                if let Some(pos) = cam_sets.last_cursor_pos {
                    window.set_cursor_position(Some(pos));
                }
                cursor_options.visible = true;
                CamState::Fixed
            }
        };
    }

    // === Navigation Check ===
    // Two ways to navigate:
    // 1. Free camera mode (toggled with Tab) - for mouse users
    // 2. Middle mouse button held - for trackpad users
    let is_navigating =
        cam_sets.cam_state == CamState::Free || mouse_buttons.pressed(MouseButton::Middle);

    if is_navigating {
        // === Speed Adjustment (Mouse Wheel) ===
        let scroll_amount = mouse_wheel.delta.y;
        if scroll_amount != 0.0 {
            cam_sets.speed += scroll_amount * cam_sets.sensitivity;
            cam_sets.speed = cam_sets
                .speed
                .clamp(cam_sets.speed_range.start, cam_sets.speed_range.end);
        }

        // === Camera Rotation (Mouse Movement) ===
        let delta = mouse_motion.delta;

        // Update target rotation based on mouse input
        cam_sets.target_yaw -= delta.x * 0.003;
        cam_sets.target_pitch -= delta.y * 0.003;
        
        // Clamp pitch to prevent looking straight up/down (gimbal lock)
        cam_sets.target_pitch = cam_sets
            .target_pitch
            .clamp(-cam_sets.pitch_limit, cam_sets.pitch_limit);

        // Get current rotation angles
        let (current_yaw, current_pitch, _) = cam.rotation.to_euler(EulerRot::YXZ);

        // Smoothly interpolate toward target rotation
        // Use angle_difference for yaw to handle wrapping at ±π
        let yaw_diff = angle_difference(cam_sets.target_yaw, current_yaw);
        let smoothed_yaw = current_yaw + yaw_diff * cam_sets.rotation_smoothing * time.delta_secs();
        let smoothed_pitch = current_pitch
            + (cam_sets.target_pitch - current_pitch)
                * cam_sets.rotation_smoothing
                * time.delta_secs();

        // Apply smoothed rotation
        cam.rotation = Quat::from_euler(EulerRot::YXZ, smoothed_yaw, smoothed_pitch, 0.0);

        // === Camera Movement (Keyboard Input) ===
        let mut target_velocity = Vec3::ZERO;
        
        // Calculate movement directions relative to camera orientation
        let forward = cam.forward();
        let forward_flattened = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        let right = cam.right();

        // Forward/Backward (W/S or Arrow keys)
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            target_velocity += forward_flattened;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            target_velocity -= forward_flattened;
        }
        
        // Left/Right (A/D or Arrow keys)
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            target_velocity -= *right;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            target_velocity += *right;
        }
        
        // Up (Space/Shift/E) - Reduced to 85% to feel less floaty
        if keyboard_input.pressed(KeyCode::Space)
            || keyboard_input.pressed(KeyCode::ShiftRight)
            || keyboard_input.pressed(KeyCode::KeyE)
        {
            target_velocity += Vec3::Y * 0.85;
        }
        
        // Down (Shift/Ctrl/Q)
        if keyboard_input.pressed(KeyCode::ShiftLeft)
            || keyboard_input.pressed(KeyCode::ControlRight)
            || keyboard_input.pressed(KeyCode::KeyQ)
        {
            target_velocity -= Vec3::Y * 0.85;
        }

        // Normalize and scale by current speed
        target_velocity = target_velocity.normalize_or_zero() * cam_sets.speed;

        // === Velocity Smoothing ===
        let delta_time = time.delta_secs();
        
        if target_velocity.length() > 0.01 {
            // Player is pressing movement keys: accelerate toward target
            cam_sets.current_velocity = cam_sets
                .current_velocity
                .lerp(target_velocity, cam_sets.acceleration * delta_time);
        } else {
            // No input: apply friction to slow down smoothly
            cam_sets.current_velocity = cam_sets
                .current_velocity
                .lerp(Vec3::ZERO, cam_sets.friction * delta_time);
        }

        // Apply smoothed velocity to camera position
        cam.translation += cam_sets.current_velocity * delta_time;

        // === Boundary Constraints ===
        // Keep camera within allowed area (board + margin)
        let min_x = -cam_sets.nav_margin;
        let max_x = cam_sets.x_limit.end + cam_sets.nav_margin;
        let min_z = -cam_sets.nav_margin;
        let max_z = cam_sets.z_limit.end + cam_sets.nav_margin;

        cam.translation.x = cam.translation.x.clamp(min_x, max_x);
        cam.translation.y = cam
            .translation
            .y
            .clamp(cam_sets.y_limit.start, cam_sets.y_limit.end);
        cam.translation.z = cam.translation.z.clamp(min_z, max_z);
    }
}