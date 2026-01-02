use bevy::prelude::*;

use crate::game::NodeType;

use super::components::{FixedCamUiRoot, HotbarSlot, NodeButton, NodeSelectionPanel};

pub fn setup_fixed_cam_ui(mut commands: Commands) {
    let root = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                padding: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
            Pickable::IGNORE,
            FixedCamUiRoot,
        ))
        .id();

    let hotbar = commands
        .spawn((
            Node {
                width: Val::Auto,
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(4.0),
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            NodeSelectionPanel,
        ))
        .id();

    for (idx, (node_type, text)) in NodeType::all().enumerate() {
        let slot = commands
            .spawn((
                Button,
                Node {
                    width: Val::Px(52.0),
                    height: Val::Px(52.0),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.8)),
                BorderColor::all(Color::srgb(0.2, 0.2, 0.2)),
                NodeButton { node_type },
                HotbarSlot,
            ))
            .id();

        let slot_text = commands
            .spawn((
                Text::new(text),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ))
            .id();

        let number_text = commands
            .spawn((
                Text::new(idx.to_string()),
                TextFont {
                    font_size: 8.0,
                    ..default()
                },
                TextColor(Color::srgba(0.8, 0.8, 0.8, 0.7)),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(2.0),
                    right: Val::Px(4.0),
                    ..default()
                },
            ))
            .id();

        commands.entity(slot).add_child(slot_text);
        commands.entity(slot).add_child(number_text);
        commands.entity(hotbar).add_child(slot);
    }

    commands.entity(root).add_child(hotbar);
}

pub fn cleanup_fixed_cam_ui(mut commands: Commands, roots: Query<Entity, With<FixedCamUiRoot>>) {
    for e in &roots {
        commands.entity(e).despawn();
    }
}
