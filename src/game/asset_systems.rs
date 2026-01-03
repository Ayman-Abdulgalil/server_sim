use super::constants::*;
use super::components::TileNodeLink;
use super::node_type::NodeType;
use crate::game::resources::RenderAssets;
use bevy::prelude::*;

/// initialize all asset handles
pub fn init_asset_handles_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile_mesh: Handle<Mesh> = asset_server.load(TILE_PATH);
    let tile_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: TILE_COLOR,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });
    let tile_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: TILE_VFX,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let internet_mesh: Handle<Mesh> = asset_server.load(INTERNET_PATH);
    let internet_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: INTERNET_COLOR,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });
    let internet_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: INTERNET_VFX,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let loadbalancer_mesh: Handle<Mesh> = asset_server.load(LOADBALANCER_PATH);
    let loadbalancer_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: LOADBALANCER_COLOR,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });
    let loadbalancer_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: LOADBALANCER_VFX,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let firewall_mesh: Handle<Mesh> = asset_server.load(FIREWALL_PATH);
    let firewall_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: FIREWALL_COLOR,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });
    let firewall_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: FIREWALL_VFX,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let database_mesh: Handle<Mesh> = asset_server.load(DATABASE_PATH);
    let database_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: DATABASE_COLOR,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });
    let database_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: DATABASE_VFX,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let compute_mesh: Handle<Mesh> = asset_server.load(COMPUTE_PATH);
    let compute_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: COMPUTE_COLOR,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });
    let compute_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: COMPUTE_VFX,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let storage_mesh: Handle<Mesh> = asset_server.load(STORAGE_PATH);
    let storage_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: STORAGE_COLOR,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });
    let storage_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: STORAGE_VFX,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let queue_mesh: Handle<Mesh> = asset_server.load(QUEUE_PATH);
    let queue_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: QUEUE_COLOR,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });
    let queue_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: QUEUE_VFX,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let cache_mesh: Handle<Mesh> = asset_server.load(CACHE_PATH);
    let cache_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: CACHE_COLOR,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });
    let cache_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: CACHE_VFX,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });

    let cdn_mesh: Handle<Mesh> = asset_server.load(CDN_PATH);
    let cdn_mat: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: CDN_COLOR,
        perceptual_roughness: 0.8,
        metallic: 0.2,
        ..default()
    });
    let cdn_vfx: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: CDN_VFX,
        perceptual_roughness: 0.8,
        metallic: 0.2,
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

/// change the material of all TileNodeLink entities based of hover status
pub fn update_hover_materials_system(
    render_assets: Res<RenderAssets>,
    changed: Query<&TileNodeLink, Changed<TileNodeLink>>,
    mut mats: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    for link in &changed {
        let tile_handle = if link.hovered {
            render_assets.tile_vfx.clone()
        } else {
            render_assets.tile_mat.clone()
        };

        let node_handle = match link.node_type.unwrap_or(NodeType::None) {
            NodeType::None => None,
            NodeType::Internet => Some(if link.hovered {
                render_assets.internet_vfx.clone()
            } else {
                render_assets.internet_mat.clone()
            }),
            NodeType::LoadBalancer => Some(if link.hovered {
                render_assets.loadbalancer_vfx.clone()
            } else {
                render_assets.loadbalancer_mat.clone()
            }),
            NodeType::Firewall => Some(if link.hovered {
                render_assets.firewall_vfx.clone()
            } else {
                render_assets.firewall_mat.clone()
            }),
            NodeType::Compute => Some(if link.hovered {
                render_assets.compute_vfx.clone()
            } else {
                render_assets.compute_mat.clone()
            }),
            NodeType::Database => Some(if link.hovered {
                render_assets.database_vfx.clone()
            } else {
                render_assets.database_mat.clone()
            }),
            NodeType::Storage => Some(if link.hovered {
                render_assets.storage_vfx.clone()
            } else {
                render_assets.storage_mat.clone()
            }),
            NodeType::Queue => Some(if link.hovered {
                render_assets.queue_vfx.clone()
            } else {
                render_assets.queue_mat.clone()
            }),
            NodeType::Cache => Some(if link.hovered {
                render_assets.cache_vfx.clone()
            } else {
                render_assets.cache_mat.clone()
            }),
            NodeType::CDN => Some(if link.hovered {
                render_assets.cdn_vfx.clone()
            } else {
                render_assets.cdn_mat.clone()
            }),
        };

        // update tile material
        if let Ok(mut tile_mat) = mats.get_mut(link.tile) {
            tile_mat.0 = tile_handle;
        }

        // update node material (if a node exists)
        if let (Some(node_e), Some(node_handle)) = (link.node, node_handle) {
            if let Ok(mut node_mat) = mats.get_mut(node_e) {
                node_mat.0 = node_handle;
            }
        }
    }
}

/// resets the material of all TileNodeLink entities to the base material
pub fn reset_hover_materials_system(
    render_assets: Res<RenderAssets>,
    changed: Query<&TileNodeLink, Changed<TileNodeLink>>,
    mut mats: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    for link in &changed {
        let tile_handle = render_assets.tile_mat.clone();

        let node_handle = match link.node_type.unwrap_or(NodeType::None) {
            NodeType::None => None,
            NodeType::Internet => Some(render_assets.internet_mat.clone()),
            NodeType::LoadBalancer => Some(render_assets.loadbalancer_mat.clone()),
            NodeType::Firewall => Some(render_assets.firewall_mat.clone()),
            NodeType::Compute => Some(render_assets.compute_mat.clone()),
            NodeType::Database => Some(render_assets.database_mat.clone()),
            NodeType::Storage => Some(render_assets.storage_mat.clone()),
            NodeType::Queue => Some(render_assets.queue_mat.clone()),
            NodeType::Cache => Some(render_assets.cache_mat.clone()),
            NodeType::CDN => Some(render_assets.cdn_mat.clone()),
        };

        // update tile material
        if let Ok(mut tile_mat) = mats.get_mut(link.tile) {
            tile_mat.0 = tile_handle;
        }

        // update node material (if a node exists)
        if let (Some(node_e), Some(node_handle)) = (link.node, node_handle) {
            if let Ok(mut node_mat) = mats.get_mut(node_e) {
                node_mat.0 = node_handle;
            }
        }
    }
}
