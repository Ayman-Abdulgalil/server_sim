use bevy::prelude::*;

use crate::game::{Game, NodeType};

use super::components::NodeButton;

pub fn handle_ui_buttons(
    mut interaction_query: Query<(&Interaction, &NodeButton), Changed<Interaction>>,
    mut game: ResMut<Game>,
) {
    // Interaction is how Bevy reports hovered/pressed/none for UI widgets. [web:50]
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            game.hotbar_selection = button.node_type;
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
        game.hotbar_selection = node_type;
    }
}
