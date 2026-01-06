use bevy::prelude::*;

use crate::game::state::GameState;

use super::{hud, setup_menu, systems};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // Setup menu only in Setup state
        app.add_systems(OnEnter(GameState::Setup), setup_menu::spawn_setup_menu)
            .add_systems(OnExit(GameState::Setup), setup_menu::despawn_setup_menu)
            .add_systems(Update, systems::setup_menu_buttons.run_if(in_state(GameState::Setup)));

        // HUD for all states except Setup (Paused/Playing/Fast/GameOver)
        app.add_systems(
            OnEnter(GameState::Paused),
            hud::ensure_hud_spawned, // spawn once when first reaching paused
        )
        .add_systems(
            Update,
            (
                systems::top_bar_buttons,
                systems::hotbar_buttons,
                systems::node_palette_buttons,
                systems::sync_hud_visibility,
            )
                .run_if(not(in_state(GameState::Setup))),
        );
    }
}
