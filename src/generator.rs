use bevy::app::{App, AppExit, Plugin, Startup, Update};
use bevy::prelude::{ButtonInput, EventWriter, KeyCode, Res};

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, keyboard_input);
    }
}

fn setup() {
    println!("Hello, world!");
}

fn keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
    }
}
