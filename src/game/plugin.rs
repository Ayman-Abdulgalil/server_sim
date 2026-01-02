use super::resources::Game;
use super::setup::init_asset_handles_system;
use super::setup::setup_game_system;
use super::state::GameState;

use crate::camera::CamPlugin;
use crate::ui::UIPlugin;

use bevy::prelude::*;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MeshPickingPlugin, CamPlugin, UIPlugin))
            .init_resource::<Game>()
            .init_state::<GameState>()
            .add_systems(
                OnEnter(GameState::Playing),
                (init_asset_handles_system, setup_game_system).chain(),
            );
    }
}
