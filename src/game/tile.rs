use super::components::*;
use super::input_click::tile_click_event;
use super::resources::RenderAssets;

use bevy::prelude::*;

pub fn spawn_tile(commands: &mut Commands, render_assets: &RenderAssets, pos: Vec3) -> Entity {
    commands
        .spawn((
            Mesh3d(render_assets.tile_mesh.clone()),
            MeshMaterial3d(render_assets.tile_mat.clone()),
            Transform::from_translation(pos),
            TileEntity,
            Pickable::default(),
        ))
        .observe(tile_click_event)
        .id()
}