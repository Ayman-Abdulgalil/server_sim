use super::types::NodeType;

use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct TileNodeLink {
    pub tile: Entity,
    pub node: Option<Entity>,
}

#[derive(Component)]
pub struct TileTag {
    pub selected: bool,
    pub base_y: f32,
    pub curr_y: f32,
}

#[derive(Component)]
pub struct NodeTag {
    pub node_type: NodeType,
    pub selected: bool,
    pub base_y: f32,
    pub curr_y: f32,
}
