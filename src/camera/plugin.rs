use bevy::prelude::*;

use crate::game::GameState;

use super::settings::CamSettings;
use super::state::CamState;
use super::systems::{cam_movement, mode_toggle, setup_camera};

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CamSettings>()
            .init_state::<CamState>()
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (mode_toggle, cam_movement).run_if(in_state(GameState::Playing)),
            );
    }
}
