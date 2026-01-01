use bevy::prelude::*;

use crate::camera::CamPlugin;
use crate::ui::UIPlugin;

use super::fx_tile_hover::animate_tile_hover_fx_system;
use super::input_click::board_click_system;
use super::resources::Game;
use super::setup::setup_game_system;
use super::state::GameState;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MeshPickingPlugin, CamPlugin, UIPlugin))
            .init_resource::<Game>()
            .init_state::<GameState>()
            .add_systems(OnEnter(GameState::Playing), setup_game_system)
            .add_systems(
                Update,
                (board_click_system, animate_tile_hover_fx_system)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
