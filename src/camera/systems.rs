use bevy::{
    input::{
        ButtonInput,
        mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    },
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::game::Game;

use super::components::MainCam;
use super::consts;
use super::movement::{
    apply_friction_and_coast, apply_smoothed_rotation, apply_speed_scroll,
    clamp_translation, compute_target_velocity, update_look_targets, update_velocity,
};
use super::settings::CamSettings;
use super::state::CamState;
use super::math::{angle_difference, exp_smooth_factor};

pub fn setup_camera(mut commands: Commands, mut cam_sets: ResMut<CamSettings>, game: Res<Game>) {
    *cam_sets = CamSettings::from_game(&game);

    let initial_transform =
        Transform::from_translation(cam_sets.cam_position).looking_at(cam_sets.cam_target, Vec3::Y);

    let (yaw, pitch, _) = initial_transform.rotation.to_euler(EulerRot::YXZ);
    cam_sets.target_yaw = yaw;
    cam_sets.target_pitch = pitch;

    commands.spawn((Camera3d::default(), MainCam, initial_transform));
}

pub fn cam_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_wheel: Res<AccumulatedMouseScroll>,
    cam_state: Res<State<CamState>>,
    mut cam_sets: ResMut<CamSettings>,
    mut cam: Single<&mut Transform, With<MainCam>>,
) {
    // AccumulatedMouseMotion/Scroll are resources that reset each frame. [web:32]
    let dt_move = time.delta_secs().min(consts::MAX_DT);
    let dt_rot = time.delta_secs();

    let in_free = *cam_state.get() == CamState::Free;
    let is_navigating = in_free || mouse_buttons.pressed(MouseButton::Middle);

    if is_navigating {
        apply_speed_scroll(mouse_wheel.delta.y, &mut cam_sets);

        let a_in = exp_smooth_factor(cam_sets.mouse_input_smoothing, dt_rot);
        cam_sets.mouse_delta_smoothed = cam_sets.mouse_delta_smoothed.lerp(mouse_motion.delta, a_in);

        update_look_targets(cam_sets.mouse_delta_smoothed, &mut cam_sets);
        cam_sets.rotating = true;

        let target_velocity = compute_target_velocity(&keyboard_input, &cam, cam_sets.speed);
        update_velocity(dt_move, target_velocity, &mut cam_sets);

        cam.translation += cam_sets.current_velocity * dt_move;
        clamp_translation(&mut cam, &cam_sets);
    } else {
        apply_friction_and_coast(dt_move, &mut cam_sets, &mut cam);

        let a_in = exp_smooth_factor(cam_sets.mouse_input_smoothing, dt_rot);
        cam_sets.mouse_delta_smoothed = cam_sets.mouse_delta_smoothed.lerp(Vec2::ZERO, a_in);
    }

    if cam_sets.rotating {
        apply_smoothed_rotation(dt_rot, &mut cam_sets, &mut cam);

        let (cy, cp, _) = cam.rotation.to_euler(EulerRot::YXZ);
        let yaw_err = angle_difference(cam_sets.target_yaw, cy).abs();
        let pitch_err = (cam_sets.target_pitch - cp).abs();

        if yaw_err < consts::ROT_STOP_EPS && pitch_err < consts::ROT_STOP_EPS {
            cam_sets.rotating = false;
        }
    }
}

pub fn mode_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    cam_state: Res<State<CamState>>,
    mut next_cam_state: ResMut<NextState<CamState>>,
    mut cam_sets: ResMut<CamSettings>,
    mut cursor_options: Single<&mut CursorOptions>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Tab) {
        return;
    }

    match *cam_state.get() {
        CamState::Fixed => {
            cursor_options.grab_mode = CursorGrabMode::Locked;
            cam_sets.last_cursor_pos = window.cursor_position();
            cursor_options.visible = false;

            next_cam_state.set(CamState::Free);
            info!("Camera set to Free. Cursor position: {:?}", cam_sets.last_cursor_pos);
        }
        CamState::Free => {
            if let Some(pos) = cam_sets.last_cursor_pos {
                window.set_cursor_position(Some(pos));
            }
            cursor_options.visible = true;
            cursor_options.grab_mode = CursorGrabMode::Confined;

            next_cam_state.set(CamState::Fixed);
            info!("Camera set to Fixed. Cursor position: {:?}", window.cursor_position());
        }
    }
}
