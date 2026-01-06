use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Setup,
    Paused,
    Playing,
    Fast,
    GameOver,
}
