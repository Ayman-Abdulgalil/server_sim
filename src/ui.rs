use bevy::prelude::*;

use crate::game::*;

// ============================================================================
// Components
// ============================================================================

#[derive(Component)]
pub struct NodeSelectionPanel;

#[derive(Component)]
pub struct NodeButton {
    pub node_type: NodeType,
}

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct HotbarSlot {}

// ============================================================================
// Setup System
// ============================================================================

pub fn setup_ui(mut commands: Commands) {
    // Root UI node
    let root = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            padding: UiRect::bottom(Val::Px(20.0)),
            ..default()
        })
        .id();

    // Hotbar container
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

    // Create slots for each node type
    for (idx, (node_type, label)) in NodeType::all_types().iter().enumerate() {
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
                NodeButton { node_type: *node_type },
                HotbarSlot {},
            ))
            .id();

        // Node type label (larger)
        let slot_text = commands
            .spawn((
                Text::new(*label),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ))
            .id();

        // Slot number (smaller, at bottom)
        let number_text = commands
            .spawn((
                Text::new((idx).to_string()),
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

    // Add hotbar to root
    commands.entity(root).add_child(hotbar);
}

// ============================================================================
// UI Interaction Systems
// ============================================================================

pub fn handle_ui_buttons(
    mut interaction_query: Query<
        (&Interaction, &NodeButton),
        Changed<Interaction>,
    >,
    mut game: ResMut<Game>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            game.selected_node = button.node_type;
            println!("Selected: {}", button.node_type.name());
        }
    }
}

pub fn handle_hotkeys(keys: Res<ButtonInput<KeyCode>>, mut game: ResMut<Game>) {
    let selection = match () {
        _ if keys.just_pressed(KeyCode::Digit1) => Some(NodeType::Internet),
        _ if keys.just_pressed(KeyCode::Digit2) => Some(NodeType::LoadBalancer),
        _ if keys.just_pressed(KeyCode::Digit3) => Some(NodeType::Firewall),
        _ if keys.just_pressed(KeyCode::Digit4) => Some(NodeType::Database),
        _ if keys.just_pressed(KeyCode::Digit5) => Some(NodeType::Compute),
        _ if keys.just_pressed(KeyCode::Digit6) => Some(NodeType::Storage),
        _ if keys.just_pressed(KeyCode::Digit7) => Some(NodeType::Queue),
        _ if keys.just_pressed(KeyCode::Digit8) => Some(NodeType::Cache),
        _ if keys.just_pressed(KeyCode::Digit9) => Some(NodeType::CDN),
        _ if keys.just_pressed(KeyCode::Digit0) => Some(NodeType::None),
        _ => None,
    };

    if let Some(node_type) = selection {
        game.selected_node = node_type;
        println!("Selected: {}", node_type.name());
    }
}

pub fn update_button_colors(
    mut buttons: Query<(&mut BackgroundColor, &mut BorderColor, &Interaction, &NodeButton)>,
    game: Res<Game>,
) {
    for (mut bg_color, mut border_color, interaction, button) in &mut buttons {
        let is_selected = button.node_type == game.selected_node;
        
        match (*interaction, is_selected) {
            (Interaction::Pressed, _) => {
                *bg_color = BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9));
                *border_color = BorderColor::all(Color::srgb(0.6, 0.6, 0.6));
            }
            (Interaction::Hovered, _) => {
                *bg_color = BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 0.9));
                *border_color = BorderColor::all(Color::srgb(0.7, 0.7, 0.7));
            }
            (Interaction::None, true) => {
                *bg_color = BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.95));
                *border_color = BorderColor::all(Color::srgb(1.0, 1.0, 1.0));
            }
            (Interaction::None, false) => {
                *bg_color = BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.8));
                *border_color = BorderColor::all(Color::srgb(0.2, 0.2, 0.2));
            }
        };
    }
}

// ============================================================================
// Game Over Systems
// ============================================================================

pub fn show_game_over_screen(mut commands: Commands) {
    let container = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            GameOverUI,
        ))
        .id();

    let text = commands
        .spawn((
            Text::new("Game Over!\nPress SPACE to restart"),
            TextFont {
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(Justify::Center),
        ))
        .id();

    commands.entity(container).add_child(text);
}

pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn handle_game_over_input(
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}