use super::components::TileNodeLink;
use super::nodes::{hover_out_event, hover_over_event, spawn_node};
use super::resources::{Game, RenderAssets};
use super::node_type::NodeType;

use crate::camera::CamState;

use bevy::prelude::*;

pub fn spawn_tile(commands: &mut Commands, render_assets: &RenderAssets, pos: Vec3) -> Entity {
    let tile_e = commands
        .spawn((
            Mesh3d(render_assets.tile_mesh.clone()),
            MeshMaterial3d(render_assets.tile_mat.clone()),
            Transform::from_translation(pos),
            Pickable::default(),
        ))
        .observe(tile_click_event)
        .observe(hover_over_event)
        .observe(hover_out_event)
        .id();

    commands.entity(tile_e).insert(TileNodeLink {
        tile: tile_e,
        node: None,
        node_type: None,
        hovered: false,
        anim_transition: 0.0,
    });

    tile_e
}

fn tile_click_event(
    mut click: On<Pointer<Click>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    render_assets: Res<RenderAssets>,
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
    
    spawn_node(&mut commands, &render_assets, &mut game, node_selection, tile_x, tile_z);

    click.propagate(false);
}
