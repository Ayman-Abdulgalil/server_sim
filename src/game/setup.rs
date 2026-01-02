use super::components::TileData;
use super::constants::*;
use super::resources::{Game, RenderAssets};
use super::tile::spawn_tile;

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
            range: 200.0,
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

/// initialize all asset handles
pub fn init_asset_handles_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile_mesh: Handle<Mesh> = asset_server.load(TILE_PATH);
    let tile_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: TILE_COLOR,
        emissive: TILE_EMISSIVE,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let internet_mesh: Handle<Mesh> = asset_server.load(INTERNET_PATH);
    let internet_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: INTERNET_COLOR,
        emissive: INTERNET_EMISSIVE,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let loadbalancer_mesh: Handle<Mesh> = asset_server.load(LOADBALANCER_PATH);
    let loadbalancer_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: LOADBALANCER_COLOR,
        emissive: LOADBALANCER_EMISSIVE,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let firewall_mesh: Handle<Mesh> = asset_server.load(FIREWALL_PATH);
    let firewall_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: FIREWALL_COLOR,
        emissive: FIREWALL_EMISSIVE,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let database_mesh: Handle<Mesh> = asset_server.load(DATABASE_PATH);
    let database_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: DATABASE_COLOR,
        emissive: DATABASE_EMISSIVE,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let compute_mesh: Handle<Mesh> = asset_server.load(COMPUTE_PATH);
    let compute_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: COMPUTE_COLOR,
        emissive: COMPUTE_EMISSIVE,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let storage_mesh: Handle<Mesh> = asset_server.load(STORAGE_PATH);
    let storage_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: STORAGE_COLOR,
        emissive: STORAGE_EMISSIVE,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let queue_mesh: Handle<Mesh> = asset_server.load(QUEUE_PATH);
    let queue_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: QUEUE_COLOR,
        emissive: QUEUE_EMISSIVE,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let cache_mesh: Handle<Mesh> = asset_server.load(CACHE_PATH);
    let cache_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: CACHE_COLOR,
        emissive: CACHE_EMISSIVE,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let cdn_mesh: Handle<Mesh> = asset_server.load(CDN_PATH);
    let cdn_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: CDN_COLOR,
        emissive: CDN_EMISSIVE,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    commands.insert_resource(RenderAssets {
        tile_mesh,
        tile_mat,
        internet_mesh,
        internet_mat,
        loadbalancer_mesh,
        loadbalancer_mat,
        firewall_mesh,
        firewall_mat,
        database_mesh,
        database_mat,
        compute_mesh,
        compute_mat,
        storage_mesh,
        storage_mat,
        queue_mesh,
        queue_mat,
        cache_mesh,
        cache_mat,
        cdn_mesh,
        cdn_mat,
    });
}
