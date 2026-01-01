use bevy::prelude::*;

use crate::camera::CamState;
use crate::game::GameState;

use super::game_over::{cleanup_game_over, handle_game_over_input, show_game_over_screen};
use super::hotbar::{cleanup_fixed_cam_ui, setup_fixed_cam_ui};
use super::hotbar_input::{handle_hotkeys, handle_ui_buttons};
use super::hotbar_style::update_button_colors;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CamState::Fixed), setup_fixed_cam_ui)
            .add_systems(OnExit(CamState::Fixed), cleanup_fixed_cam_ui)
            .add_systems(
                Update,
                (handle_ui_buttons, handle_hotkeys, update_button_colors)
                    .run_if(in_state(CamState::Fixed)),
            )
            // Optional: wire up game-over UI now that you already have the systems.
            .add_systems(OnEnter(GameState::GameOver), show_game_over_screen)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
            .add_systems(
                Update,
                handle_game_over_input.run_if(in_state(GameState::GameOver)),
            );
    }
}
