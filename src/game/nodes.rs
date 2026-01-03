use super::components::TileNodeLink;
use super::constants::*;
use super::node_type::NodeType;
use super::resources::{Game, RenderAssets};

use crate::camera::CamState;

use bevy::prelude::*;

pub fn spawn_node(
    commands: &mut Commands,
    render_assets: &RenderAssets,
    game: &mut Game,
    node_selection: NodeType,
    x: usize,
    z: usize,
) {
    let Some((mesh, mat, _vfx)) = render_assets.get_node_assets(node_selection) else {
        game.board[z][x].node_entity = None;
        return;
    };

    let tile_e = game.board[z][x].tile_entity;

    let node_e = commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(mat),
            Transform::from_xyz(x as f32, NODE_SPAWN_Y, z as f32),
            Pickable::default(),
        ))
        .observe(node_click_event)
        .observe(hover_over_event)
        .observe(hover_out_event)
        .id();

    // Store the relationship on both entities.
    let component = TileNodeLink {
        tile: tile_e,
        node: Some(node_e),
        node_type: Some(node_selection),
        hovered: false,
        anim_transition: 0.0,
    };
    commands.entity(node_e).insert(component);
    commands.entity(tile_e).insert(component);

    game.board[z][x].node_entity = Some(node_e);
}

pub fn node_click_event(
    mut click: On<Pointer<Click>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    buttons: Res<ButtonInput<MouseButton>>,
    cam_state: Res<State<CamState>>,
    ui_query: Query<&Interaction, With<Button>>,
    link_gt: Query<&GlobalTransform, With<TileNodeLink>>,
    mut links: Query<&mut TileNodeLink>,
) {
    if click.event.button != PointerButton::Primary {
        return;
    }

    // check if the click landed on a ui element, camera is in Free mode or the middle button is pressed
    let should_skip: bool;
    if ui_query.iter().any(|i| *i != Interaction::None) {
        should_skip = true;
    } else if *cam_state == CamState::Free || buttons.pressed(MouseButton::Middle) {
        should_skip = true;
    } else {
        should_skip = false;
    }

    if should_skip {
        click.propagate(false);
        return;
    }

    // get clicked node world position
    let Ok(global_transform) = link_gt.get(click.entity) else {
        return;
    };
    let world_position: Vec3 = global_transform.translation();

    // convert to board coords
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

    let node_selection = game.hotbar_selection;
    if node_selection == NodeType::None {
        // delete node
        let Some(entity) = game.board[tile_z][tile_x].node_entity else {
            return;
        };

        let tile_e = game.board[tile_z][tile_x].tile_entity;
        if let Ok(mut tile_link) = links.get_mut(tile_e) {
            tile_link.node = None;
            tile_link.node_type = None;
            tile_link.hovered = false;
            tile_link.anim_transition = 0.0;
        }

        commands.entity(entity).despawn();
        game.board[tile_z][tile_x].node_entity = None;
        return;
    }
}

pub fn hover_over_event(
    over: On<Pointer<Over>>,
    mut commands: Commands,
    q_links: Query<&TileNodeLink>,
) {
    if let Ok(link) = q_links.get(over.entity) {
        commands.entity(link.tile).insert(TileNodeLink {
            tile: link.tile,
            node: link.node,
            node_type: link.node_type,
            hovered: true,
            anim_transition: 0.0,
        });
        if let Some(node) = link.node {
            commands.entity(node).insert(TileNodeLink {
                tile: link.tile,
                node: link.node,
                node_type: link.node_type,
                hovered: true,
                anim_transition: 0.0,
            });
        }
    }
}

pub fn hover_out_event(
    out: On<Pointer<Out>>,
    mut commands: Commands,
    q_links: Query<&TileNodeLink>,
) {
    if let Ok(link) = q_links.get(out.entity) {
        commands.entity(out.entity).insert(TileNodeLink {
            tile: link.tile,
            node: link.node,
            node_type: link.node_type,
            hovered: false,
            anim_transition: 0.0,
        });
    }
}
