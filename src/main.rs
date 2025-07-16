mod items;
mod power;
mod tools;
mod ui;
mod wire_system;
mod world;

use crate::items::ItemsPlugin;
use crate::power::power::PowerPlugin;
use crate::ui::UiPlugin;
use crate::wire_system::WireSystemPlugin;
use crate::world::camera::CameraPlugin;
use bevy::prelude::*;
use tools::build_tool::BuildToolPlugin;
use world::grid::GridPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Option::from(Window {
                title: "Bevy Sandbox".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ItemsPlugin)
        .add_plugins((GridPlugin, BuildToolPlugin, WireSystemPlugin, PowerPlugin))
        .add_plugins((UiPlugin, CameraPlugin))
        // .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .run();
}

// fn setup(mut commands: Commands) {
// commands.spawn(Camera2d);
// }

fn keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
    }
}
