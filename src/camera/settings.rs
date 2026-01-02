use bevy::prelude::*;
use std::{f32::consts::FRAC_PI_2, ops::Range};

use crate::game::Game;

use super::constants;

#[derive(Resource, Default)]
pub struct CamSettings {
    pub cam_position: Vec3,
    pub cam_target: Vec3,

    pub nav_margin: f32,
    pub x_limit: Range<f32>,
    pub y_limit: Range<f32>,
    pub z_limit: Range<f32>,

    pub speed: f32,
    pub speed_range: Range<f32>,

    pub sensitivity: f32,

    pub acceleration: f32,
    pub friction: f32,
    pub current_velocity: Vec3,

    pub pitch_limit: f32,
    pub target_yaw: f32,
    pub target_pitch: f32,

    pub rotation_smoothing: f32,
    pub mouse_input_smoothing: f32,
    pub mouse_delta_smoothed: Vec2,

    pub rotating: bool,

    pub last_cursor_pos: Option<Vec2>,

    pub rot_speed_reference: f32,
    pub rot_scale_range: Range<f32>,
}

impl CamSettings {
    pub fn from_game(game: &Game) -> Self {
        let board_center = Vec3::new(
            game.board_size_x as f32 / 2.0,
            0.0,
            game.board_size_z as f32 / 2.0,
        );

        let cam_position = Vec3::new(
            game.board_size_x as f32 / 2.0,
            2.0 * game.board_size_z as f32 / 3.0,
            game.board_size_z as f32 / 2.0 + 10.0,
        );

        let nav_margin = 8.0;

        Self {
            cam_target: board_center,
            cam_position,

            nav_margin,
            x_limit: -nav_margin..32.0,
            y_limit: 2.0..50.0,
            z_limit: -nav_margin..32.0,

            speed: 10.0,
            speed_range: 1.0..32.0,

            sensitivity: 4.0,
            acceleration: 6.0,
            friction: 6.0,
            current_velocity: Vec3::ZERO,

            pitch_limit: FRAC_PI_2 - constants::PITCH_EPS,
            target_yaw: 0.0,
            target_pitch: 0.0,

            rotation_smoothing: 12.0,
            mouse_input_smoothing: 35.0,
            mouse_delta_smoothed: Vec2::ZERO,

            rotating: false,
            last_cursor_pos: None,

            rot_speed_reference: 10.0,
            rot_scale_range: 0.35..2.5,
        }
    }

    pub fn bounds_with_margin(&self) -> (f32, f32, f32, f32) {
        let min_x = -self.nav_margin;
        let max_x = self.x_limit.end + self.nav_margin;
        let min_z = -self.nav_margin;
        let max_z = self.z_limit.end + self.nav_margin;
        (min_x, max_x, min_z, max_z)
    }

    pub fn rotation_scale(&self) -> f32 {
        (self.speed / self.rot_speed_reference)
            .clamp(self.rot_scale_range.start, self.rot_scale_range.end)
    }
}
