use bevy::prelude::*;

use super::components::*;
use super::constants::*;

pub fn attach_node_select_fx(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    node: Entity,
) {
    let mat = materials.add(StandardMaterial {
        emissive: LinearRgba::rgb(0.0, 2.5, 2.5),
        ..default()
    });

    let fx = commands
        .spawn((
            Mesh3d(meshes.add(Circle::new(NODE_SELECT_RING_RADIUS))),
            MeshMaterial3d(mat),
            Transform::from_translation(Vec3::new(0.0, NODE_SELECT_RING_Y, 0.0))
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            Visibility::Hidden,
            NodeSelectFx,
        ))
        .id();

    commands.entity(node).add_child(fx);
}

pub fn node_click_toggle_select(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut vis_fx: Query<&mut Visibility, With<NodeSelectFx>>,
    selected: Query<Entity, With<SelectedNode>>,
    children: Query<&Children>,
) {
    for e in &selected {
        commands.entity(e).remove::<SelectedNode>();
        set_node_fx_visibility(e, Visibility::Hidden, &children, &mut vis_fx);
    }

    commands.entity(click.entity).insert(SelectedNode);
    set_node_fx_visibility(click.entity, Visibility::Visible, &children, &mut vis_fx);
}

fn set_node_fx_visibility(
    node: Entity,
    visibility: Visibility,
    children: &Query<&Children>,
    vis_fx: &mut Query<&mut Visibility, With<NodeSelectFx>>,
) {
    let Ok(ch) = children.get(node) else { return; };

    for &c in ch {
        if let Ok(mut v) = vis_fx.get_mut(c) {
            *v = visibility;
        }
    }
}
