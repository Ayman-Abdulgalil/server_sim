use bevy::prelude::*;

use super::components::BoardTile;
use super::node_type::NodeType;

#[derive(Resource, Default)]
pub struct Game {
    pub board: Vec<Vec<BoardTile>>,
    pub board_size_x: usize,
    pub board_size_y: usize,
    pub hotbar_selection: NodeType,
}
