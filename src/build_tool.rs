use crate::ui::MainText;
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

fn build_tool_selector(
    keys: Res<ButtonInput<KeyCode>>,
    mut build_tool: ResMut<BuildTool>,
    mut text: Query<&mut Text, With<MainText>>,
) {
    for mut t in text.iter_mut() {
        if keys.just_pressed(KeyCode::Digit1) {
            *build_tool = BuildTool::Generator;

            **t = "Generator".into();
        }

        if keys.just_pressed(KeyCode::Digit2) {
            *build_tool = BuildTool::PowerPole;

            **t = "Power Pole".into();
        }

        if keys.just_pressed(KeyCode::Digit3) {
            *build_tool = BuildTool::Light;

            **t = "Light".into();
        }
    }
}
