use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod camera;
use camera::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource, Default)]
struct Game {
    board: Vec<Vec<Cell>>,
    board_size_x: usize,
    board_size_y: usize,
}


struct Cell {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .init_state::<GameState>()
        .init_resource::<CamSettings>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(
            Update,
            (
                handle_mouse_click,
                cam_movement.run_if(in_state(GameState::Playing)),
            ),
        )
        .add_systems(OnEnter(GameState::GameOver), game_over_screen)
        .add_systems(
            Update,
            game_over_keyboard.run_if(in_state(GameState::GameOver)),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut cam_sets: ResMut<CamSettings>, // Must be ResMut to modify
) {
    game.board_size_x = 32;
    game.board_size_y = 32;

    // Correctly setting values using Vec3
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

    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 4.0),
    ));

    let tile: Handle<Scene> = asset_server.load("models/Tile.glb#Scene0");

    game.board = (0..game.board_size_x)
        .map(|j| {
            (0..game.board_size_y)
                .map(|i| {
                    commands.spawn((
                        Transform::from_xyz(i as f32, 0.0, j as f32),
                        SceneRoot(tile.clone()),
                    ));
                    Cell {}
                })
                .collect()
        })
        .collect();
}

fn game_over_keyboard(
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn game_over_screen() { /* UI Logic */
}

// Assuming your board is at Y = 0
fn ray_plane_intersection(ray: &Ray3d, plane_y: f32) -> Option<Vec3> {
    // Check if ray is parallel to plane
    if ray.direction.y.abs() < 0.0001 {
        return None;
    }

    // Calculate intersection distance
    let t = (plane_y - ray.origin.y) / ray.direction.y;

    // Check if intersection is in front of camera
    if t < 0.0 {
        return None;
    }

    // Calculate intersection point
    Some(ray.origin + ray.direction * t)
}

fn handle_mouse_click(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera.into_inner();

        // Get cursor position in window
        if let Some(cursor_pos) = window.cursor_position() {
            // Convert screen position to ray
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                // ray.origin = camera position
                // ray.direction = normalized direction vector

                println!("Ray origin: {:?}", ray.origin);
                println!("Ray direction: {:?}", ray.direction);

                if let Some(hit_point) = ray_plane_intersection(&ray, 0.0) {
                    println!("Clicked at: x={}, z={}", hit_point.x, hit_point.z);

                    // Convert to board coordinates
                    let board_x = hit_point.x as i32;
                    let board_z = hit_point.z as i32;

                    println!("Board coordinates: ({}, {})", board_x, board_z);
                }
            }
        }
    }
}
