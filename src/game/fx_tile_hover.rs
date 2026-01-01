use bevy::prelude::*;
use bevy::color::Mix;

use super::components::*;
use super::constants::*;

pub fn spawn_tile_with_hover_fx(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
) -> Entity {
    let tile_mesh: Handle<Mesh> = asset_server.load("models/Tile.glb#Mesh0/Primitive0");

    // Per-tile material so we can animate glow per tile.
    let tile_mat = materials.add(StandardMaterial {
        base_color: TILE_BASE_COLOR,
        emissive: TILE_BASE_EMISSIVE,
        ..default()
    });

    let hover_ring_mat = materials.add(StandardMaterial {
        emissive: LinearRgba::rgb(2.5, 2.5, 0.0),
        ..default()
    });

    let ring = commands
        .spawn((
            Mesh3d(meshes.add(Circle::new(TILE_HOVER_RING_RADIUS))),
            MeshMaterial3d(hover_ring_mat),
            Transform::from_translation(Vec3::new(0.0, TILE_HOVER_RING_Y, 0.0))
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            Visibility::Hidden,
            TileHoverFx,
        ))
        .id();

    commands
        .spawn((
            Mesh3d(tile_mesh),
            MeshMaterial3d(tile_mat),
            Transform::from_translation(pos),
            TileBaseY(pos.y),
            TileHoverState { hovered: false },
            TileHoverAnim { t: 0.0 },
            Pickable::default(),
            TileRoot,
        ))
        .add_child(ring)
        .observe(tile_over_event)
        .observe(tile_out_event)
        .id()
}

fn tile_over_event(
    mut over: On<Pointer<Over>>,
    mut q_state: Query<&mut TileHoverState, With<TileRoot>>,
    mut q_fx: Query<&mut Visibility, With<TileHoverFx>>,
    children: Query<&Children>,
) {
    if let Ok(mut s) = q_state.get_mut(over.entity) {
        s.hovered = true;
    }

    if let Ok(ch) = children.get(over.entity) {
        for &c in ch {
            if let Ok(mut v) = q_fx.get_mut(c) {
                *v = Visibility::Visible;
            }
        }
    }

    over.propagate(false);
}

fn tile_out_event(
    mut out: On<Pointer<Out>>,
    mut q_state: Query<&mut TileHoverState, With<TileRoot>>,
    mut q_fx: Query<&mut Visibility, With<TileHoverFx>>,
    children: Query<&Children>,
) {
    if let Ok(mut s) = q_state.get_mut(out.entity) {
        s.hovered = false;
    }

    if let Ok(ch) = children.get(out.entity) {
        for &c in ch {
            if let Ok(mut v) = q_fx.get_mut(c) {
                *v = Visibility::Hidden;
            }
        }
    }

    out.propagate(false);
}

pub fn animate_tile_hover_fx_system(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_tiles: Query<(
        &TileBaseY,
        &TileHoverState,
        &mut TileHoverAnim,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
    ), With<TileRoot>>,
) {
    let dt = time.delta_secs();

    for (base_y, state, mut anim, mut transform, mat_handle) in &mut q_tiles {
        let target = if state.hovered { 1.0 } else { 0.0 };

        let k = 1.0 - (-TILE_HOVER_SPEED * dt).exp();
        anim.t = anim.t + (target - anim.t) * k;
        anim.t = anim.t.clamp(0.0, 1.0);

        transform.translation.y = base_y.0 + TILE_HOVER_LIFT_Y * anim.t;

        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            let base_lin: LinearRgba = TILE_BASE_COLOR.into();
            let hover_lin: LinearRgba = TILE_HOVER_COLOR.into();
            let mixed_lin: LinearRgba = base_lin.mix(&hover_lin, anim.t);
            mat.base_color = Color::from(mixed_lin);

            mat.emissive = TILE_BASE_EMISSIVE.mix(&TILE_HOVER_EMISSIVE, anim.t);
        }
    }
}
