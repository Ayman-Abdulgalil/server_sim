use bevy::prelude::*;

mod camera;
mod game;
mod ui;

use camera::*;
use game::*;
use ui::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .init_state::<GameState>()
        .init_resource::<CamSettings>()
        .add_systems(Startup, (setup_camera, setup_ui))
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(
            Update,
            (
                handle_hotkeys,
                handle_board_click,
                handle_node_click,
                handle_ui_buttons,
                update_button_colors,
                cam_movement,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), show_game_over_screen)
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
        .add_systems(
            Update,
            handle_game_over_input.run_if(in_state(GameState::GameOver)),
        )
        .run();
}
