use super::components::TileNodeLink;
use super::constants::{SPAWN_FALL_Y, TILE_SPAWN_Y};
use super::nodes::spawn_node;
use super::resources::{Game, RenderAssets};

use crate::camera::CamState;
use crate::game::components::{NodeTag, TileTag};
use crate::game::types::ToolType;

use bevy::prelude::*;

pub fn spawn_tile(commands: &mut Commands, render_assets: &RenderAssets, pos: Vec3) -> Entity {
    let tile_e = commands
        .spawn((
            Mesh3d(render_assets.tile_mesh.clone()),
            MeshMaterial3d(render_assets.tile_mat.clone()),
            Transform::from_xyz(pos.x, pos.y + SPAWN_FALL_Y, pos.z),
            Pickable::default(),
        ))
        .observe(tile_click_event)
        .observe(tile_hover_over_event)
        .observe(tile_hover_out_event)
        .id();

    commands.entity(tile_e).insert((
        TileNodeLink {
            tile: tile_e,
            node: None,
        },
        TileTag {
            selected: false,
            base_y: TILE_SPAWN_Y,
            curr_y: TILE_SPAWN_Y + SPAWN_FALL_Y,
        },
    ));

    tile_e
}

fn tile_click_event(
    mut click: On<Pointer<Click>>,
    mut commands: Commands,
    game: ResMut<Game>,
    render_assets: Res<RenderAssets>,
    buttons: Res<ButtonInput<MouseButton>>,
    cam_state: Res<State<CamState>>,
    mut tile_links: Query<&mut TileNodeLink, With<TileTag>>,
) {
    if click.event.button != PointerButton::Primary {
        return;
    }

    // Skip world clicks while free cam / middle mouse.
    if *cam_state == CamState::Free || buttons.pressed(MouseButton::Middle) {
        click.propagate(false);
        return;
    }

    // Use the picking hit position.
    let Some(world_position) = click.event.hit.position else {
        click.propagate(false);
        return;
    };

    // Convert to board coords.
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

    // Must be a tile (has TileTag + TileNodeLink).
    let Ok(mut link) = tile_links.get_mut(click.entity) else {
        return;
    };

    // Consume the click once we know it's a valid tile interaction.
    click.propagate(false);

    match game.tool_selection {
        ToolType::Delete => {
            if let Some(node_e) = link.node {
                commands.entity(node_e).despawn();
                link.node = None;
            }
        }

        ToolType::Add(node_type) => {
            if link.node.is_none() {
                spawn_node(
                    &mut commands,
                    &mut link,
                    &render_assets,
                    node_type,
                    tile_x,
                    tile_z,
                );
            }
        }
        _ => {}
    }
}

fn tile_hover_over_event(
    over: On<Pointer<Over>>,
    render_assets: Res<RenderAssets>,
    mut tiles: Query<(&TileNodeLink, &mut MeshMaterial3d<StandardMaterial>), (With<TileTag>, Without<NodeTag>)>,
    mut nodes: Query<(&NodeTag, &mut MeshMaterial3d<StandardMaterial>), (With<NodeTag>, Without<TileTag>)>,
) {
    let Ok((link, mut tile_m)) = tiles.get_mut(over.entity) else {
        return;
    };
    tile_m.0 = render_assets.tile_vfx.clone();

    let Some(node_e) = link.node else {
        return;
    };
    let Ok((node_tag, mut node_m)) = nodes.get_mut(node_e) else {
        return;
    };

    let (_mesh, _mat, vfx) = render_assets.get_node_assets(node_tag.node_type);
    node_m.0 = vfx.clone();
}

fn tile_hover_out_event(
    out: On<Pointer<Out>>,
    render_assets: Res<RenderAssets>,
    mut tiles: Query<(&TileNodeLink, &mut MeshMaterial3d<StandardMaterial>), (With<TileTag>, Without<NodeTag>)>,
    mut nodes: Query<(&NodeTag, &mut MeshMaterial3d<StandardMaterial>), (With<NodeTag>, Without<TileTag>)>,
) {
    let Ok((link, mut tile_m)) = tiles.get_mut(out.entity) else {
        return;
    };
    tile_m.0 = render_assets.tile_mat.clone();

    let Some(node_e) = link.node else {
        return;
    };
    let Ok((node_tag, mut node_m)) = nodes.get_mut(node_e) else {
        return;
    };

    let (_mesh, mat, _vfx) = render_assets.get_node_assets(node_tag.node_type);
    node_m.0 = mat.clone();
}
