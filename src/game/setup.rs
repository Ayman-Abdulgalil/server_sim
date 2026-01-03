use super::components::TileData;
use super::constants::*;
use super::resources::{Game, RenderAssets};
use super::tiles::spawn_tile;

use bevy::prelude::*;

/// setup the game board and lights
pub fn setup_game_system(
    mut commands: Commands,
    render_assets: Res<RenderAssets>,
    mut game: ResMut<Game>,
) {
    game.board_size_x = GAME_BOARD_SIZE_X;
    game.board_size_z = GAME_BOARD_SIZE_Z;

    // set light
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
            radius: 20.0,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));

    // set board
    game.board = (0..game.board_size_z)
        .map(|z| {
            (0..game.board_size_x)
                .map(|x| {
                    let entity = spawn_tile(
                        &mut commands,
                        &render_assets,
                        Vec3::new(x as f32, TILE_SPAWN_Y, z as f32),
                    );
                    TileData {
                        tile_entity: entity,
                        node_entity: None,
                    }
                })
                .collect()
        })
        .collect();
}
