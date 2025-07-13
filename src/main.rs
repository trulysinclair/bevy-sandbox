mod generator;

use bevy::prelude::*;
use crate::generator::GeneratorPlugin;

fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Option::from(Window {
            title: "Bevy Sandbox".into(),
            ..default()
        }),
        ..default()
    })).add_plugins(GeneratorPlugin).add_systems(Startup,setup).run();
}

fn setup(
    mut commands: Commands,
){
    commands.spawn(Camera2d);
}