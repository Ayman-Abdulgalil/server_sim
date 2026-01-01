use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera::CamState;

use super::components::NodeEntity;
use super::constants::*;
use super::fx_node_select::{attach_node_select_fx, node_click_toggle_select};
use super::node_type::NodeType;
use super::resources::Game;


pub fn board_click_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    buttons: Res<ButtonInput<MouseButton>>,
    cam_state: Res<State<CamState>>,
    mut game: ResMut<Game>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    ui_query: Query<&Interaction, With<Button>>,
) {
    if should_skip_click(&buttons, cam_state.get(), &ui_query) {
        return;
    }

    let Some(cursor_pos) = window.cursor_position() else { return; };

    let (camera, camera_transform) = camera.into_inner();
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else { return; };

    let plane_origin = Vec3::new(0.0, CLICK_PLANE_Y, 0.0);
    let Some(dist) = ray.intersect_plane(plane_origin, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };
    let hit = ray.get_point(dist);

    let Some((tile_x, tile_y)) = world_pos_to_tile(hit, game.board_size_x, game.board_size_y)
    else {
        return;
    };

    place_node(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        &mut game,
        tile_x,
        tile_y,
    );
}

fn should_skip_click(
    buttons: &ButtonInput<MouseButton>,
    cam_state: &CamState,
    ui_query: &Query<&Interaction, With<Button>>,
) -> bool {
    if ui_query.iter().any(|i| *i != Interaction::None) {
        return true;
    }
    if *cam_state == CamState::Free || buttons.pressed(MouseButton::Middle) {
        return true;
    }
    !buttons.just_pressed(MouseButton::Left)
}

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

fn place_node(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    game: &mut Game,
    x: usize,
    y: usize,
) {
    let selected = game.hotbar_selection;

    // Tile occupied.
    if let Some(existing) = game.board[y][x].entity {
        if selected == NodeType::None {
            commands.entity(existing).despawn();
            game.board[y][x].entity = None;
            game.board[y][x].node_type = NodeType::None;
        }
        return;
    }

    // Empty tile.
    if selected == NodeType::None {
        game.board[y][x].node_type = NodeType::None;
        return;
    }

    let Some(path) = selected.mesh_path() else {
        game.board[y][x].node_type = NodeType::None;
        return;
    };

    game.board[y][x].node_type = selected;

    let mesh: Handle<Mesh> = asset_server.load(path);
    let node_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.85, 0.9),
        ..default()
    });

    let node = commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(node_mat),
            Transform::from_xyz(x as f32, NODE_Y, y as f32),
            NodeEntity,
            Pickable::default(),
        ))
        .observe(node_click_toggle_select)
        .id();

    attach_node_select_fx(commands, meshes, materials, node);
    game.board[y][x].entity = Some(node);
}
