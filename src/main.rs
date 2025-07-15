mod generator;
mod grid;
mod build_tool;
mod ui;

use crate::grid::GridPlugin;
use bevy::prelude::*;
use crate::build_tool::BuildToolPlugin;
use crate::generator::GeneratorPlugin;
use crate::ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Option::from(Window {
                title: "Bevy Sandbox".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GeneratorPlugin, GridPlugin, BuildToolPlugin))
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
