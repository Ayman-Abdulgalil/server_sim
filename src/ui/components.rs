use bevy::prelude::*;

use crate::game::NodeType;

#[derive(Component)]
pub struct NodeButton {
    pub node_type: NodeType,
}

#[derive(Component)]
pub struct NodeSelectionPanel;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct HotbarSlot;

#[derive(Component)]
pub struct FixedCamUiRoot;
