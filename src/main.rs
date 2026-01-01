use bevy::{prelude::*, window::{CursorOptions, CursorGrabMode}};

mod game;
mod camera;
mod ui;

use crate::game::GameLogicPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameLogicPlugin)
        .add_systems(Startup, setup_window)
        .run();
}

fn setup_window(mut cursor_options: Single<&mut CursorOptions>,) {
    cursor_options.grab_mode = CursorGrabMode::Confined;
}