// use crate::game::shader_materials::HoverVfxMaterial;
use super::components::TileData;
use super::node_type::NodeType;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Game {
    pub board: Vec<Vec<TileData>>,
    pub board_size_x: usize,
    pub board_size_z: usize,
    pub hotbar_selection: NodeType,
}

/// this resource stores all game handles (Mesh, Materials, Shaders, etc)
/// mainly for instancing and gpu acceleration (I think)
#[derive(Resource, Clone)]
pub struct RenderAssets {
    pub tile_mesh: Handle<Mesh>,
    pub tile_mat: Handle<StandardMaterial>,
    pub internet_mesh: Handle<Mesh>,
    pub internet_mat: Handle<StandardMaterial>,
    pub loadbalancer_mesh: Handle<Mesh>,
    pub loadbalancer_mat: Handle<StandardMaterial>,
    pub firewall_mesh: Handle<Mesh>,
    pub firewall_mat: Handle<StandardMaterial>,
    pub database_mesh: Handle<Mesh>,
    pub database_mat: Handle<StandardMaterial>,
    pub compute_mesh: Handle<Mesh>,
    pub compute_mat: Handle<StandardMaterial>,
    pub storage_mesh: Handle<Mesh>,
    pub storage_mat: Handle<StandardMaterial>,
    pub queue_mesh: Handle<Mesh>,
    pub queue_mat: Handle<StandardMaterial>,
    pub cache_mesh: Handle<Mesh>,
    pub cache_mat: Handle<StandardMaterial>,
    pub cdn_mesh: Handle<Mesh>,
    pub cdn_mat: Handle<StandardMaterial>,
    // pub tile_hover_vfx_mat: Handle<HoverVfxMaterial>,
}

impl RenderAssets {
    /// look up the mesh+material handles for a node type.
    pub fn get_node_assets(
        &self,
        node: NodeType,
    ) -> Option<(Handle<Mesh>, Handle<StandardMaterial>)> {
        match node {
            NodeType::None => None,
            NodeType::Internet => Some((self.internet_mesh.clone(), self.internet_mat.clone())),
            NodeType::LoadBalancer => Some((
                self.loadbalancer_mesh.clone(),
                self.loadbalancer_mat.clone(),
            )),
            NodeType::Firewall => Some((self.firewall_mesh.clone(), self.firewall_mat.clone())),
            NodeType::Database => Some((self.database_mesh.clone(), self.database_mat.clone())),
            NodeType::Compute => Some((self.compute_mesh.clone(), self.compute_mat.clone())),
            NodeType::Storage => Some((self.storage_mesh.clone(), self.storage_mat.clone())),
            NodeType::Queue => Some((self.queue_mesh.clone(), self.queue_mat.clone())),
            NodeType::Cache => Some((self.cache_mesh.clone(), self.cache_mat.clone())),
            NodeType::CDN => Some((self.cdn_mesh.clone(), self.cdn_mat.clone())),
        }
    }
}
