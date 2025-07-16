use crate::build_tool::BuildTool;
use bevy::prelude::*;

pub struct UiPlugin;

#[derive(Component)]
pub struct MainText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, tool: Res<BuildTool>) {
    let tool_name: &str = match *tool {
        BuildTool::Generator => "Generator",
        BuildTool::PowerPole => "Power Pole",
        BuildTool::Light => "Light",
        BuildTool::Wire => "Wire",
    };

    commands
        .spawn((
            Text::new("Tool: "),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(100.0),
                bottom: Val::Px(100.0),
                ..default()
            },
            MainText,
        ))
        .with_child((TextSpan::new(tool_name),));
}
