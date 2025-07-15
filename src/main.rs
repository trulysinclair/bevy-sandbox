mod build_tool;
mod grid;
mod items;
mod ui;
mod wire_system;

use crate::build_tool::BuildToolPlugin;
use crate::grid::GridPlugin;
use crate::items::ItemsPlugin;
use crate::ui::UiPlugin;
use crate::wire_system::WireSystemPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Option::from(Window {
                title: "Bevy Sandbox".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((ItemsPlugin))
        .add_plugins((GridPlugin, BuildToolPlugin, WireSystemPlugin))
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
    }
}
