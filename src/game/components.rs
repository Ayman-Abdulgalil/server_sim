use bevy::prelude::*;

/// Represents a single tile on the game board
#[derive(Component, Clone, Copy, Debug)]
pub struct TileData {
    /// The type of node placed on this tile, if any.
    pub tile_entity: Entity,
    /// The node entity placed on this tile, if any.
    pub node_entity: Option<Entity>,
}

/// Marker component on node entities
#[derive(Component, Clone, Copy, Debug)]
pub struct NodeEntity;

#[derive(Component, Clone, Copy, Debug)]
pub struct TileEntity;

/// Marker component on tile hover VFX entities
#[derive(Component)]
pub struct TileHoverVFX;

/// Marker component on selected node VFX entities
#[derive(Component)]
pub struct NodeSelectionVFX;

/// Marker component on selected node entities
#[derive(Component)]
pub struct SelectedNode;
