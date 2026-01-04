use super::constants::*;
use crate::game::components::{NodeTag, TileTag};
use crate::game::resources::RenderAssets;
use bevy::prelude::*;

pub fn init_asset_handles_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile_mesh: Handle<Mesh> = asset_server.load(TILE_PATH);
    let tile_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: TILE_COLOR,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });
    let tile_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: TILE_VFX,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });

    let internet_mesh: Handle<Mesh> = asset_server.load(INTERNET_PATH);
    let internet_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: INTERNET_COLOR,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });
    let internet_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: INTERNET_VFX,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });

    let loadbalancer_mesh: Handle<Mesh> = asset_server.load(LOADBALANCER_PATH);
    let loadbalancer_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: LOADBALANCER_COLOR,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });
    let loadbalancer_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: LOADBALANCER_VFX,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });

    let firewall_mesh: Handle<Mesh> = asset_server.load(FIREWALL_PATH);
    let firewall_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: FIREWALL_COLOR,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });
    let firewall_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: FIREWALL_VFX,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });

    let database_mesh: Handle<Mesh> = asset_server.load(DATABASE_PATH);
    let database_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: DATABASE_COLOR,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });
    let database_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: DATABASE_VFX,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });

    let compute_mesh: Handle<Mesh> = asset_server.load(COMPUTE_PATH);
    let compute_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: COMPUTE_COLOR,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });
    let compute_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: COMPUTE_VFX,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });

    let storage_mesh: Handle<Mesh> = asset_server.load(STORAGE_PATH);
    let storage_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: STORAGE_COLOR,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });
    let storage_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: STORAGE_VFX,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });

    let queue_mesh: Handle<Mesh> = asset_server.load(QUEUE_PATH);
    let queue_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: QUEUE_COLOR,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });
    let queue_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: QUEUE_VFX,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });

    let cache_mesh: Handle<Mesh> = asset_server.load(CACHE_PATH);
    let cache_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: CACHE_COLOR,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });
    let cache_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: CACHE_VFX,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });

    let cdn_mesh: Handle<Mesh> = asset_server.load(CDN_PATH);
    let cdn_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: CDN_COLOR,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });
    let cdn_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: CDN_VFX,
        perceptual_roughness: 0.5,
        metallic: 0.5,
        ..default()
    });

    commands.insert_resource(RenderAssets {
        tile_mesh,
        tile_mat,
        tile_vfx,

        internet_mesh,
        internet_mat,
        internet_vfx,

        loadbalancer_mesh,
        loadbalancer_mat,
        loadbalancer_vfx,

        firewall_mesh,
        firewall_mat,
        firewall_vfx,

        database_mesh,
        database_mat,
        database_vfx,

        compute_mesh,
        compute_mat,
        compute_vfx,

        storage_mesh,
        storage_mat,
        storage_vfx,

        queue_mesh,
        queue_mat,
        queue_vfx,

        cache_mesh,
        cache_mat,
        cache_vfx,

        cdn_mesh,
        cdn_mat,
        cdn_vfx,
    });
}

pub fn reset_hover_materials_system(
    render_assets: Res<RenderAssets>,
    mut tile_mats: Query<&mut MeshMaterial3d<StandardMaterial>, (With<TileTag>, Without<NodeTag>)>,
    mut node_mats: Query<
        (&NodeTag, &mut MeshMaterial3d<StandardMaterial>),
        (With<NodeTag>, Without<TileTag>),
    >,
) {
    for mut tile_m in &mut tile_mats {
        tile_m.0 = render_assets.tile_mat.clone();
    }

    for (node, mut node_m) in &mut node_mats {
        let (_mesh, mat, _vfx) = render_assets.get_node_assets(node.node_type);
        node_m.0 = mat
    }
}

pub fn update_selection_lift_system(
    time: Res<Time>,
    mut tiles: Query<(&mut Transform, &mut TileTag), (With<TileTag>, Without<NodeTag>)>,
    mut nodes: Query<(&mut Transform, &mut NodeTag), (With<NodeTag>, Without<TileTag>)>,
) {
    let dt = time.delta_secs();

    for (mut tf, mut tag) in &mut tiles {
        let target = if tag.selected { TILE_LIFT_Y } else { 0.0 };
        tag.curr_y = tag.curr_y + (target - tag.curr_y) * (HOVER_LIFT_SPEED * dt).clamp(0.0, 1.0);
        tf.translation.y = tag.base_y + tag.curr_y;
    }

    for (mut tf, mut tag) in &mut nodes {
        let target = if tag.selected { NODE_HOVER_LIFT } else { 0.0 };
        tag.curr_y = tag.curr_y + (target - tag.curr_y) * (HOVER_LIFT_SPEED * dt).clamp(0.0, 1.0);
        tf.translation.y = tag.base_y + tag.curr_y;
    }
}
