use super::components::{NodeEntity, TileEntity};
use super::constants::*;
use super::node_type::NodeType;
use super::resources::{Game, RenderAssets};

use crate::camera::CamState;

use bevy::prelude::*;

pub fn tile_click_event(
    mut click: On<Pointer<Click>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    render_assets: Res<RenderAssets>,
    buttons: Res<ButtonInput<MouseButton>>,
    cam_state: Res<State<CamState>>,
    ui_query: Query<&Interaction, With<Button>>,
    tile_gt: Query<&GlobalTransform, With<TileEntity>>,
) {
    if click.event.button != PointerButton::Primary {
        return;
    }

    if should_skip_click(&buttons, cam_state.get(), &ui_query) {
        click.propagate(false);
        return;
    }

    // get clicked tile world position
    let Ok(global_transform) = tile_gt.get(click.entity) else {
        return;
    };
    let world_position: Vec3 = global_transform.translation();

    // convert to board coords
    let Some((x, z)) = world_pos_to_tile(world_position, game.board_size_x, game.board_size_z)
    else {
        click.propagate(false);
        return;
    };

    place_node(&mut commands, &render_assets, &mut game, x, z);

    click.propagate(false);
}

fn node_click_event(
    mut click: On<Pointer<Click>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    buttons: Res<ButtonInput<MouseButton>>,
    cam_state: Res<State<CamState>>,
    ui_query: Query<&Interaction, With<Button>>,
    node_gt: Query<&GlobalTransform, With<NodeEntity>>,
) {
    if click.event.button != PointerButton::Primary {
        return;
    }

    if should_skip_click(&buttons, cam_state.get(), &ui_query) {
        click.propagate(false);
        return;
    }

    let Ok(global_transform) = node_gt.get(click.entity) else {
        return;
    };
    let world_position: Vec3 = global_transform.translation();

    // convert to board coords
    let Some((x, z)) = world_pos_to_tile(world_position, game.board_size_x, game.board_size_z)
    else {
        click.propagate(false);
        return;
    };

    let node_selection = game.hotbar_selection;
    if node_selection == NodeType::None {
        // delete node
        let Some(entity) = game.board[z][x].node_entity else {
            return;
        };
        commands.entity(entity).despawn();
        game.board[z][x].node_entity = None;
        return;
    }
}

/// place an instance of a node in a tile depending on the hotbar selection
fn place_node(
    commands: &mut Commands,
    render_assets: &RenderAssets,
    game: &mut Game,
    x: usize,
    z: usize,
) {
    let node_selection = game.hotbar_selection;

    // Tile occupied.
    if let Some(entity) = game.board[z][x].node_entity {
        if node_selection == NodeType::None {
            // delete node
            commands.entity(entity).despawn();
            game.board[z][x].node_entity = None;
            return;
        }
        return;
    }

    // Empty tile.
    if node_selection == NodeType::None {
        return;
    }

    game.board[z][x].node_entity = spawn_node(commands, render_assets, node_selection, x, z);
}

fn spawn_node(
    commands: &mut Commands,
    render_assets: &RenderAssets,
    node_selection: NodeType,
    x: usize,
    z: usize,
) -> Option<Entity> {
    let Some(assets) = render_assets.get_node_assets(node_selection) else {
        return None;
    };
    let mesh = assets.0;
    let mat = assets.1;

    Some(
        commands
            .spawn((
                Mesh3d(mesh),
                MeshMaterial3d(mat),
                Transform::from_xyz(x as f32, NODE_SPAWN_Y, z as f32),
                NodeEntity,
                Pickable::default(),
            ))
            .observe(node_click_event)
            .id(),
    )
}

/// check if the click should be ignored
fn should_skip_click(
    buttons: &ButtonInput<MouseButton>,
    cam_state: &CamState,
    ui_query: &Query<&Interaction, With<Button>>,
) -> bool {
    // check if the click landed on a ui element
    if ui_query.iter().any(|i| *i != Interaction::None) {
        return true;
    }
    // check if the camera is in Free mode or the middle button is pressed
    if *cam_state == CamState::Free || buttons.pressed(MouseButton::Middle) {
        return true;
    }
    false
}

/// round a world position to the nearest board tile center
fn world_pos_to_tile(hit: Vec3, size_x: usize, size_y: usize) -> Option<(usize, usize)> {
    let tile_x = hit.x.round() as i32;
    let tile_y = hit.z.round() as i32;

    if tile_x < 0 || tile_y < 0 {
        return None;
    }

    let (tile_x, tile_y) = (tile_x as usize, tile_y as usize);
    if tile_x >= size_x || tile_y >= size_y {
        return None;
    }

    Some((tile_x, tile_y))
}
