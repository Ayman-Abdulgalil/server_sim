use bevy::prelude::*;

use crate::game::GameState;

use super::components::GameOverUI;

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
