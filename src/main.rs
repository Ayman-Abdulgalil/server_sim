use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowPlugin, WindowPosition, WindowResolution},
};

mod camera;
mod game;
mod ui;

use crate::game::GameLogicPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "server_sim".into(),
                resolution: WindowResolution::new(1080, 720),
                resizable: true,
                decorations: true,
                transparent: false,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                mode: WindowMode::Windowed,
                present_mode: PresentMode::AutoVsync,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GameLogicPlugin)
        .run();
}
