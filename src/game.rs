use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera::*;

// ============================================================================
// Game State & Resources
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource, Default)]
pub struct Game {
    pub board: Vec<Vec<BoardTile>>,
    pub board_size_x: usize,
    pub board_size_y: usize,
    pub selected_node: NodeType,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BoardTile {
    pub node_type: NodeType,
    pub entity: Option<Entity>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum NodeType {
    #[default]
    None,
    Internet,
    LoadBalancer,
    Firewall,
    Database,
    Compute,
    Storage,
    Queue,
    Cache,
    CDN,
}

impl NodeType {
    pub fn model_path(&self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Internet => Some("models/Internet.glb#Scene0"),
            Self::LoadBalancer => Some("models/LoadBalancer.glb#Scene0"),
            Self::Firewall => Some("models/Firewall.glb#Scene0"),
            Self::Database => Some("models/Database.glb#Scene0"),
            Self::Compute => Some("models/Compute.glb#Scene0"),
            Self::Storage => Some("models/Storage.glb#Scene0"),
            Self::Queue => Some("models/Queue.glb#Scene0"),
            Self::Cache => Some("models/Cache.glb#Scene0"),
            Self::CDN => Some("models/CDN.glb#Scene0"),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "Dlt",
            Self::Internet => "W.W.W",
            Self::LoadBalancer => "LB",
            Self::Firewall => "FW",
            Self::Database => "DB",
            Self::Compute => "Cmput",
            Self::Storage => "Strg",
            Self::Queue => "Que",
            Self::Cache => "Cch",
            Self::CDN => "CDN",
        }
    }

    pub fn all_types() -> [(NodeType, &'static str); 10] {
        [
            (NodeType::None, "Dlt"),
            (NodeType::Internet, "W.W.W"),
            (NodeType::LoadBalancer, "LB"),
            (NodeType::Firewall, "FW"),
            (NodeType::Database, "DB"),
            (NodeType::Compute, "Cmput"),
            (NodeType::Storage, "Strg"),
            (NodeType::Queue, "Que"),
            (NodeType::Cache, "Cch"),
            (NodeType::CDN, "CDN"),
        ]
    }
}

// ============================================================================
// Components
// ============================================================================

#[derive(Component)]
pub struct BoardCell {}

#[derive(Component)]
pub struct NodeEntity {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct Clickable {
    pub half_size: Vec3,
}

// ============================================================================
// Setup System
// ============================================================================

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut cam_sets: ResMut<CamSettings>,
) {
    game.board_size_x = 32;
    game.board_size_y = 32;

    // Initialize camera settings
    cam_sets.cam_state = CamState::Fixed;
    let board_center = Vec3::new(
        game.board_size_x as f32 / 2.0,
        0.0,
        game.board_size_y as f32 / 2.0,
    );
    cam_sets.cam_target = board_center;
    cam_sets.cam_position = Vec3::new(
        game.board_size_x as f32 / 2.0,
        2.0 * game.board_size_y as f32 / 3.0,
        game.board_size_y as f32 / 2.0 + 10.0,
    );

    // Spawn lightings
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(
            game.board_size_x as f32 / 2.0,
            20.0,
            game.board_size_y as f32 / 2.0,
        ),
    ));
    commands.spawn((
        PointLight {
            intensity: 5_000_000.0,
            shadows_enabled: true,
            range: 200.0,
            radius: 20.0,
            ..default()
        },
        Transform::from_xyz(
            game.board_size_x as f32 / 2.0,
            30.0,
            game.board_size_y as f32 / 2.0,
        )
    ));

    // Load tile model
    let tile: Handle<Scene> = asset_server.load("models/Tile.glb#Scene0");

    // Generate board with tiles
    game.board = (0..game.board_size_y)
        .map(|y| {
            (0..game.board_size_x)
                .map(|x| {
                    commands.spawn((
                        Transform::from_xyz(x as f32, 0.0, y as f32),
                        SceneRoot(tile.clone()),
                        BoardCell {},
                    ));
                    BoardTile {
                        node_type: NodeType::None,
                        entity: None,
                    }
                })
                .collect()
        })
        .collect();
}

// ============================================================================
// Click Handling Systems
// ============================================================================

pub fn handle_node_click(
    buttons: Res<ButtonInput<MouseButton>>,
    cam_sets: Res<CamSettings>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    nodes: Query<(&NodeEntity, &Transform, &Clickable)>,
    ui_query: Query<&Interaction, With<Button>>,
) {
    if should_skip_click(&buttons, &cam_sets, &ui_query) {
        return;
    }

    let (camera, camera_transform) = camera.into_inner();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else {
        return;
    };

    let mut closest_hit: Option<(f32, &NodeEntity)> = None;

    for (node, transform, clickable) in &nodes {
        if let Some(distance) =
            ray_box_intersection(&ray, transform.translation, clickable.half_size)
        {
            if closest_hit.map_or(true, |(d, _)| distance < d) {
                closest_hit = Some((distance, node));
            }
        }
    }

    if let Some((_, node)) = closest_hit {
        println!("Clicked node at ({}, {})", node.x, node.y);
    }
}

pub fn handle_board_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    buttons: Res<ButtonInput<MouseButton>>,
    cam_sets: Res<CamSettings>,
    mut game: ResMut<Game>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    nodes: Query<(&NodeEntity, &Transform, &Clickable)>,
    ui_query: Query<&Interaction, With<Button>>,
) {
    if should_skip_click(&buttons, &cam_sets, &ui_query) {
        return;
    }

    let (camera, camera_transform) = camera.into_inner();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else {
        return;
    };

    // Don't place if clicking on an existing node
    for (_, transform, clickable) in &nodes {
        if ray_box_intersection(&ray, transform.translation, clickable.half_size).is_some() {
            return;
        }
    }

    // Check for tile placement
    if let Some(hit_point) = ray_plane_intersection(&ray, 0.25) {
        let tile_x = hit_point.x.round() as i32;
        let tile_y = hit_point.z.round() as i32;

        if tile_x >= 0
            && tile_x < game.board_size_x as i32
            && tile_y >= 0
            && tile_y < game.board_size_y as i32
        {
            place_node(
                &mut commands,
                &asset_server,
                &mut game,
                tile_x as usize,
                tile_y as usize,
            );
        }
    }
}

fn should_skip_click(
    buttons: &ButtonInput<MouseButton>,
    cam_sets: &CamSettings,
    ui_query: &Query<&Interaction, With<Button>>,
) -> bool {
    // Skip if hovering over UI
    for interaction in ui_query {
        if *interaction != Interaction::None {
            return true;
        }
    }

    // Skip if in free camera mode or middle mouse is pressed
    if cam_sets.cam_state == CamState::Free || buttons.pressed(MouseButton::Middle) {
        return true;
    }

    // Skip if left mouse not just pressed
    !buttons.just_pressed(MouseButton::Left)
}

fn place_node(
    commands: &mut Commands,
    asset_server: &AssetServer,
    game: &mut Game,
    x: usize,
    y: usize,
) {
    println!("Placing {:?} at ({}, {})", game.selected_node, x, y);

    // Remove existing node if present
    if let Some(entity) = game.board[y][x].entity {
        commands.entity(entity).despawn();
    }

    // Update board state
    game.board[y][x].node_type = game.selected_node;
    game.board[y][x].entity = None;

    // Spawn new node if not None
    if game.selected_node != NodeType::None {
        if let Some(model_path) = game.selected_node.model_path() {
            let model: Handle<Scene> = asset_server.load(model_path);

            let entity = commands
                .spawn((
                    Transform::from_xyz(x as f32, 0.26, y as f32),
                    SceneRoot(model),
                    NodeEntity { x, y },
                    Clickable {
                        half_size: Vec3::new(0.4, 0.5, 0.4),
                    },
                ))
                .id();

            game.board[y][x].entity = Some(entity);
        }
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

fn ray_plane_intersection(ray: &Ray3d, plane_y: f32) -> Option<Vec3> {
    if ray.direction.y.abs() < 0.0001 {
        return None;
    }

    let t = (plane_y - ray.origin.y) / ray.direction.y;

    if t < 0.0 {
        return None;
    }

    Some(ray.origin + ray.direction * t)
}

fn ray_box_intersection(ray: &Ray3d, box_center: Vec3, half_size: Vec3) -> Option<f32> {
    let box_min = box_center - half_size;
    let box_max = box_center + half_size;

    let inv_dir = Vec3::new(
        1.0 / ray.direction.x,
        1.0 / ray.direction.y,
        1.0 / ray.direction.z,
    );

    let t1 = (box_min.x - ray.origin.x) * inv_dir.x;
    let t2 = (box_max.x - ray.origin.x) * inv_dir.x;
    let t3 = (box_min.y - ray.origin.y) * inv_dir.y;
    let t4 = (box_max.y - ray.origin.y) * inv_dir.y;
    let t5 = (box_min.z - ray.origin.z) * inv_dir.z;
    let t6 = (box_max.z - ray.origin.z) * inv_dir.z;

    let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
    let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

    if tmax < 0.0 || tmin > tmax {
        return None;
    }

    Some(if tmin < 0.0 { tmax } else { tmin })
}