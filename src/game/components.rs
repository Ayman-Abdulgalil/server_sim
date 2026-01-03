use super::node_type::NodeType;

use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct TileData {
    pub tile_entity: Entity,
    pub node_entity: Option<Entity>,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct TileNodeLink {
    pub tile: Entity,
    pub node: Option<Entity>,
    pub node_type: Option<NodeType>,

    pub hovered: bool,
    pub anim_transition: f32,
}
