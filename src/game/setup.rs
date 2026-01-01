use bevy::prelude::*;

use super::components::BoardTile;
use super::constants::*;
use super::fx_tile_hover::spawn_tile_with_hover_fx;
use super::node_type::NodeType;
use super::resources::Game;

pub fn setup_game_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
) {
    game.board_size_x = BOARD_SIZE_X;
    game.board_size_y = BOARD_SIZE_Y;

    // Lights
    let mid_x = game.board_size_x as f32 / 2.0;
    let mid_z = game.board_size_y as f32 / 2.0;
    spawn_lights(&mut commands, mid_x, mid_z);

    // Board data + tiles
    game.board = (0..game.board_size_y)
        .map(|y| {
            (0..game.board_size_x)
                .map(|x| {
                    spawn_tile_with_hover_fx(
                        &mut commands,
                        &asset_server,
                        &mut meshes,
                        &mut materials,
                        Vec3::new(x as f32, TILE_Y, y as f32),
                    );

                    BoardTile {
                        node_type: NodeType::None,
                        entity: None,
                    }
                })
                .collect()
        })
        .collect();
}

fn spawn_lights(commands: &mut Commands, mid_x: f32, mid_z: f32) {
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
}
