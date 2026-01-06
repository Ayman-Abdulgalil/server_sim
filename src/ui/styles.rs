use bevy::prelude::*;

pub const PANEL_BG: Color = Color::srgba(0.0, 0.0, 0.0, 0.6);
pub const BTN_IDLE: Color = Color::srgba(0.2, 0.2, 0.2, 0.85);
pub const BTN_HOVER: Color = Color::srgba(0.3, 0.3, 0.3, 0.95);
pub const BTN_ACTIVE: Color = Color::srgba(0.2, 0.6, 0.2, 0.95);

pub fn root_fullscreen() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }
}

pub fn text_style(size: f32) -> (TextFont, TextColor) {
    (TextFont { font_size: size, ..default() }, TextColor(Color::WHITE))
}
