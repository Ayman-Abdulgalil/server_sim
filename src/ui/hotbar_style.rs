use bevy::prelude::*;

use crate::game::Game;
use crate::game::types::ToolType;

use super::components::NodeButton;

pub fn update_button_colors(
    mut buttons: Query<(
        &mut BackgroundColor,
        &mut BorderColor,
        &Interaction,
        &NodeButton,
    )>,
    game: Res<Game>,
) {
    for (mut bg_color, mut border_color, interaction, button) in &mut buttons {
        if let ToolType::Add(node_type) = game.tool_selection {
            let is_selected = button.node_type == node_type;
    
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
}
