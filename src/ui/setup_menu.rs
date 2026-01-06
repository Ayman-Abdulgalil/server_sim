use bevy::prelude::*;
use super::styles::*;

#[derive(Component)]
pub struct SetupMenuRoot;

#[derive(Component)]
pub enum SetupButton {
    BoardXMinus,
    BoardXPlus,
    BoardZMinus,
    BoardZPlus,
    Start,
}

pub fn spawn_setup_menu(mut commands: Commands) {
    let root = commands
        .spawn((root_fullscreen(), SetupMenuRoot))
        .id();

    let panel = commands
        .spawn((
            Node {
                width: Val::Px(520.0),
                height: Val::Px(280.0),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(PANEL_BG),
        ))
        .id();

    let title = commands.spawn((Text::new("Setup Game"), text_style(28.0).0, text_style(28.0).1)).id();

    let row_x = spawn_row(&mut commands, "Board X", SetupButton::BoardXMinus, SetupButton::BoardXPlus);
    let row_z = spawn_row(&mut commands, "Board Z", SetupButton::BoardZMinus, SetupButton::BoardZPlus);

    let start_btn = spawn_button(&mut commands, "Start", SetupButton::Start);

    commands.entity(panel).add_child(title);
    commands.entity(panel).add_child(row_x);
    commands.entity(panel).add_child(row_z);
    commands.entity(panel).add_child(start_btn);

    commands.entity(root).add_child(panel);
}

fn spawn_row(commands: &mut Commands, label: &str, minus: SetupButton, plus: SetupButton) -> Entity {
    let row = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(52.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();

    let label_e = commands.spawn((Text::new(label), text_style(18.0).0, text_style(18.0).1)).id();
    let minus_btn = spawn_button(commands, "-", minus);
    let plus_btn = spawn_button(commands, "+", plus);

    commands.entity(row).add_child(label_e);
    commands.entity(row).add_child(minus_btn);
    commands.entity(row).add_child(plus_btn);
    row
}

fn spawn_button(commands: &mut Commands, text: &str, action: SetupButton) -> Entity {
    let btn = commands
        .spawn((
            Button,
            Node {
                width: Val::Px(140.0),
                height: Val::Px(44.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BTN_IDLE),
            action,
        ))
        .id();

    let t = commands.spawn((Text::new(text), text_style(18.0).0, text_style(18.0).1)).id();
    commands.entity(btn).add_child(t);
    btn
}

pub fn despawn_setup_menu(mut commands: Commands, q: Query<Entity, With<SetupMenuRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
