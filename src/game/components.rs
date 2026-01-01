use bevy::prelude::*;

use super::node_type::NodeType;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BoardTile {
    pub node_type: NodeType,
    pub entity: Option<Entity>, // Node entity (not the tile entity).
}

#[derive(Component)]
pub struct NodeEntity;

#[derive(Component)]
pub struct TileRoot;

#[derive(Component)]
pub struct TileHoverFx;

#[derive(Component)]
pub struct NodeSelectFx;

#[derive(Component)]
pub struct SelectedNode;

/// Stores the original tile Y so hover lift can always return to base.
#[derive(Component, Clone, Copy, Debug)]
pub struct TileBaseY(pub f32);

#[derive(Component, Clone, Copy, Debug)]
pub struct TileHoverState {
    pub hovered: bool,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct TileHoverAnim {
    /// 0.0 = not hovered, 1.0 = hovered
    pub t: f32,
}
