use super::styles::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct TopBar;

#[derive(Component)]
pub struct ToolHotbar;

#[derive(Component)]
pub struct NodePaletteBar;

#[derive(Component)]
pub enum TopBarButton {
    Pause,
    Play,
    Fast,
    Reset,
    Save,
    Quit,
}

#[derive(Component)]
pub enum ToolButton {
    Select,
    Add,
    Delete,
    Link,
    Move,
}

#[derive(Component)]
pub struct NodeButton(pub crate::game::NodeType);

pub fn ensure_hud_spawned(mut commands: Commands, existing: Query<Entity, With<HudRoot>>) {
    if !existing.is_empty() {
        return;
    }

    commands.spawn((root_fullscreen(), HudRoot)).with_children(|root| {
        // Top bar
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(52.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            TopBar,
        ))
        .with_children(|top| {
            // left group
            top.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(6.0),
                ..default()
            })
            .with_children(|left| {
                spawn_small_button(left, "Pause", TopBarButton::Pause);
                spawn_small_button(left, "Play", TopBarButton::Play);
                spawn_small_button(left, "Fast", TopBarButton::Fast);
            });

            // right group
            top.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(6.0),
                ..default()
            })
            .with_children(|right| {
                spawn_small_button(right, "Reset", TopBarButton::Reset);
                spawn_small_button(right, "Save", TopBarButton::Save);
                spawn_small_button(right, "Quit", TopBarButton::Quit);
            });
        });

        // Bottom tool hotbar
        root.spawn((
            Node {
                width: Val::Auto,
                height: Val::Px(64.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Percent(50.0),
                padding: UiRect::all(Val::Px(6.0)),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            ToolHotbar,
        ))
        .with_children(|bar| {
            spawn_tool_slot(bar, "Sel", ToolButton::Select);
            spawn_tool_slot(bar, "Add", ToolButton::Add);
            spawn_tool_slot(bar, "Del", ToolButton::Delete);
            spawn_tool_slot(bar, "Link", ToolButton::Link);
            spawn_tool_slot(bar, "Move", ToolButton::Move);
        });

        // Node palette
        root.spawn((
            Node {
                width: Val::Auto,
                height: Val::Px(56.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(92.0),
                left: Val::Percent(50.0),
                padding: UiRect::all(Val::Px(6.0)),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            Visibility::Hidden,
            NodePaletteBar,
        ))
        .with_children(|bar| {
            for (node_type, label) in crate::game::NodeType::all() {
                spawn_node_slot(bar, label, NodeButton(node_type));
            }
        });
    });
}

// Update these helpers to take the child spawner from with_children, not &mut Commands:
fn spawn_small_button(parent: &mut bevy::ecs::prelude::ChildSpawnerCommands, label: &str, action: TopBarButton) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(86.0),
                height: Val::Px(34.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BTN_IDLE),
            action,
        ))
        .with_children(|btn| {
            btn.spawn((Text::new(label), text_style(14.0).0, text_style(14.0).1));
        });
}

fn spawn_tool_slot(parent: &mut bevy::ecs::prelude::ChildSpawnerCommands, label: &str, action: ToolButton) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(54.0),
                height: Val::Px(54.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BTN_IDLE),
            action,
        ))
        .with_children(|btn| {
            btn.spawn((Text::new(label), text_style(14.0).0, text_style(14.0).1));
        });
}

fn spawn_node_slot(parent: &mut bevy::ecs::prelude::ChildSpawnerCommands, label: &str, action: NodeButton) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(54.0),
                height: Val::Px(44.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BTN_IDLE),
            action,
        ))
        .with_children(|btn| {
            btn.spawn((Text::new(label), text_style(12.0).0, text_style(12.0).1));
        });
}
