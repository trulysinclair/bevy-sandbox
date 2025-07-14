use bevy::prelude::*;

pub struct BuildToolPlugin;

impl Plugin for BuildToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildTool::Generator)
            .add_systems(Update, build_tool_selector);
    }
}

#[derive(Resource, PartialEq, Eq, Clone, Copy)]
pub enum BuildTool {
    Generator,
    PowerPole,
    Light,
}

#[derive(Component, Clone, Copy)]
pub enum TileContent {
    Generator,
    PowerPole,
    Light,
}

fn build_tool_selector(keys: Res<ButtonInput<KeyCode>>, mut build_tool: ResMut<BuildTool>) {
    if keys.just_pressed(KeyCode::Digit1) {
        *build_tool = BuildTool::Generator;
        println!("BuildTool: Generator");
    }

    if keys.just_pressed(KeyCode::Digit2) {
        *build_tool = BuildTool::PowerPole;
        println!("BuildTool: PowerPole");
    }

    if keys.just_pressed(KeyCode::Digit3) {
        *build_tool = BuildTool::Light;
        println!("BuildTool: Light");
    }
}
