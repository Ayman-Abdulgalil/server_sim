use super::constants::*;
use super::resources::{Game, RenderAssets};
use super::tiles::spawn_tile;

use std::{thread, time::Duration};
use bevy::prelude::*;

/// setup the game board and lights
pub fn setup_game_system(
    mut commands: Commands,
    render_assets: Res<RenderAssets>,
    mut game: ResMut<Game>,
) {
    game.board_size_x = GAME_BOARD_SIZE_X;
    game.board_size_z = GAME_BOARD_SIZE_Z;

    // spawn lights
    let mid_x = game.board_size_x as f32 / 2.0;
    let mid_z = game.board_size_z as f32 / 2.0;
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(mid_x, 20.0, mid_z),
    ));
    commands.spawn((
        PointLight {
            intensity: 5_000_000.0,
            shadows_enabled: true,
            range: 200.0,
            radius: 20.0,
            ..default()
        },
        Transform::from_xyz(mid_x, 30.0, mid_z),
    ));
    commands.spawn((
        PointLight {
            intensity: 5_000_000.0,
            shadows_enabled: true,
            range: 500.0,
            radius: 30.0,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));

    // spawn board
    for z in 0..game.board_size_z {
        for x in 0..game.board_size_x {
            spawn_tile(
                &mut commands,
                &render_assets,
                Vec3::new(x as f32, TILE_SPAWN_Y, z as f32),
            );
            // thread::sleep(Duration::from_millis(TILE_SPAWN_DELAY));
        }
    }
}
