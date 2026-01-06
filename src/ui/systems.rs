use bevy::prelude::*;
use crate::game::resources::Game;
use crate::game::state::GameState;
use crate::game::types::{ToolType, NodeType};

use super::hud::*;
use super::setup_menu::SetupButton;
use super::styles::*;

pub fn setup_menu_buttons(
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
    mut q: Query<(&Interaction, &SetupButton, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action, mut bg) in &mut q {
        *bg = match *interaction {
            Interaction::Hovered => BTN_HOVER.into(),
            Interaction::Pressed => BTN_ACTIVE.into(),
            Interaction::None => BTN_IDLE.into(),
        };

        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            SetupButton::BoardXMinus => game.board_size_x = game.board_size_x.saturating_sub(1).max(1),
            SetupButton::BoardXPlus => game.board_size_x += 1,
            SetupButton::BoardZMinus => game.board_size_z = game.board_size_z.saturating_sub(1).max(1),
            SetupButton::BoardZPlus => game.board_size_z += 1,
            SetupButton::Start => next_state.set(GameState::Paused),
        }
    }
}

pub fn top_bar_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut q: Query<(&Interaction, &TopBarButton, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action, mut bg) in &mut q {
        *bg = match *interaction {
            Interaction::Hovered => BTN_HOVER.into(),
            Interaction::Pressed => BTN_ACTIVE.into(),
            Interaction::None => BTN_IDLE.into(),
        };

        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            TopBarButton::Pause => next_state.set(GameState::Paused),
            TopBarButton::Play => next_state.set(GameState::Playing),
            TopBarButton::Fast => next_state.set(GameState::Fast),
            TopBarButton::Reset => next_state.set(GameState::Setup),
            TopBarButton::Save => {
                // TODO: emit Save event
            }
            TopBarButton::Quit => {
                // TODO: emit AppExit or go to Setup/GameOver
            }
        }
    }
}

pub fn hotbar_buttons(
    mut game: ResMut<Game>,
    mut q: Query<(&Interaction, &ToolButton, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action, mut bg) in &mut q {
        if *interaction == Interaction::Pressed {
            game.tool_selection = match action {
                ToolButton::Select => ToolType::Select,
                ToolButton::Add => ToolType::Add(NodeType::Internet),
                ToolButton::Delete => ToolType::Delete,
                ToolButton::Link => ToolType::Link,
                ToolButton::Move => ToolType::Move,
            };
        }

        // basic hover styling
        *bg = match *interaction {
            Interaction::Hovered => BTN_HOVER.into(),
            Interaction::Pressed => BTN_ACTIVE.into(),
            Interaction::None => BTN_IDLE.into(),
        };
    }
}

pub fn node_palette_buttons(
    mut game: ResMut<Game>,
    mut q: Query<(&Interaction, &NodeButton, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, node_btn, mut bg) in &mut q {
        if *interaction == Interaction::Pressed {
            game.tool_selection = ToolType::Add(node_btn.0);
        }

        *bg = match *interaction {
            Interaction::Hovered => BTN_HOVER.into(),
            Interaction::Pressed => BTN_ACTIVE.into(),
            Interaction::None => BTN_IDLE.into(),
        };
    }
}

pub fn sync_hud_visibility(
    game: Res<Game>,
    nodebar: Option<Single<&Visibility, With<NodePaletteBar>>>,
) {
    let show_nodes = matches!(game.tool_selection, ToolType::Add(_));

    if let Some(mut vis) = nodebar {
        *vis = if show_nodes { &Visibility::Visible } else { &Visibility::Hidden };
    }
}
