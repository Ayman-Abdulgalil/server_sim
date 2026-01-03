use super::asset_systems::{
    init_asset_handles_system, reset_hover_materials_system, update_hover_materials_system,
};
use super::resources::Game;
use super::setup::setup_game_system;
use super::state::GameState;

use crate::camera::{CamPlugin, CamState};
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
            )
            .add_systems(
                Update,
                update_hover_materials_system.run_if(in_state(CamState::Fixed)),
            )
            .add_systems(OnEnter(CamState::Free), reset_hover_materials_system);
    }
}
