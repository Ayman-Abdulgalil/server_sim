use super::components::{NodeTag, TileNodeLink, TileTag};
use super::constants::*;
use super::resources::{Game, RenderAssets};
use super::types::{NodeType, ToolType};

use crate::camera::CamState;

use bevy::prelude::*;

pub fn spawn_node(
    commands: &mut Commands,
    link: &mut TileNodeLink,
    render_assets: &RenderAssets,
    node_type: NodeType,
    x: usize,
    z: usize,
) {
    let (mesh, mat, _vfx) = render_assets.get_node_assets(node_type);

    let node_e = commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(mat),
            Transform::from_xyz(x as f32, NODE_SPAWN_Y + SPAWN_FALL_Y, z as f32),
            Pickable::default(),
        ))
        .observe(node_click_event)
        .observe(node_hover_over_event)
        .observe(node_hover_out_event)
        .id();

    commands.entity(node_e).insert((
        TileNodeLink {
            tile: link.tile,
            node: Some(node_e),
        },
        NodeTag {
            selected: false,
            node_type: node_type,
            base_y: NODE_SPAWN_Y,
            curr_y: NODE_SPAWN_Y + SPAWN_FALL_Y,
        },
    ));

    link.node = Some(node_e);
}

fn node_click_event(
    mut click: On<Pointer<Click>>,
    mut commands: Commands,
    game: ResMut<Game>,
    buttons: Res<ButtonInput<MouseButton>>,
    cam_state: Res<State<CamState>>,
    mut links: Query<&mut TileNodeLink>,
) {
    if click.event.button != PointerButton::Primary {
        return;
    }

    // Skip if free-cam, or middle mouse pressed.
    if *cam_state == CamState::Free || buttons.pressed(MouseButton::Middle) {
        click.propagate(false);
        return;
    }

    if game.tool_selection != ToolType::Delete {
        return;
    }

    // Compute board coords from picking hit position.
    let Some(world_position) = click.event.hit.position else {
        click.propagate(false);
        return;
    };
    let tile_x = world_position.x.round() as i32;
    let tile_z = world_position.z.round() as i32;
    if tile_x < 0 || tile_z < 0 {
        click.propagate(false);
        return;
    }
    let (tile_x, tile_z) = (tile_x as usize, tile_z as usize);
    if tile_x >= game.board_size_x || tile_z >= game.board_size_z {
        click.propagate(false);
        return;
    }

    let Ok(node_link) = links.get(click.entity) else {
        return;
    };

    let Some(node_e) = node_link.node else {
        return;
    };
    let tile_e = node_link.tile;
    click.propagate(false);
    commands.entity(node_e).despawn();
    if let Ok(mut tile_l) = links.get_mut(tile_e) {
        tile_l.node = None;
    }
}

fn node_hover_over_event(
    over: On<Pointer<Over>>,
    render_assets: Res<RenderAssets>,
    mut nodes: Query<
        (
            &NodeTag,
            &TileNodeLink,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        (With<NodeTag>, Without<TileTag>),
    >,
    mut tiles: Query<&mut MeshMaterial3d<StandardMaterial>, (With<TileTag>, Without<NodeTag>)>,
) {
    let Ok((node_tag, link, mut node_m)) = nodes.get_mut(over.entity) else {
        return;
    };

    // Hover the node itself (pick vfx by node type)
    let (_mesh, _mat, vfx) = render_assets.get_node_assets(node_tag.node_type);
    node_m.0 = vfx.clone();

    // Hover the linked tile
    if let Ok(mut tile_m) = tiles.get_mut(link.tile) {
        tile_m.0 = render_assets.tile_vfx.clone();
    }
}

fn node_hover_out_event(
    out: On<Pointer<Out>>,
    render_assets: Res<RenderAssets>,
    mut nodes: Query<
        (
            &NodeTag,
            &TileNodeLink,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        (With<NodeTag>, Without<TileTag>),
    >,
    mut tiles: Query<&mut MeshMaterial3d<StandardMaterial>, (With<TileTag>, Without<NodeTag>)>,
) {
    let Ok((node_tag, link, mut node_m)) = nodes.get_mut(out.entity) else {
        return;
    };

    // Unhover node -> base material by node type
    let (_mesh, mat, _vfx) = render_assets.get_node_assets(node_tag.node_type);
    node_m.0 = mat.clone();

    // Unhover linked tile -> base tile material
    if let Ok(mut tile_m) = tiles.get_mut(link.tile) {
        tile_m.0 = render_assets.tile_mat.clone();
    }
}
