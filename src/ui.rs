use bevy::prelude::*;
use crate::build_tool::BuildTool;

pub struct UiPlugin;

#[derive(Component)]
pub struct MainText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, tool: Res<BuildTool>) {
    let text: &str = match *tool {
        BuildTool::Generator => "Generator".into(),
        BuildTool::PowerPole => "Power Pole".into(),
        BuildTool::Light => "Light".into()
    };

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(100.0),
            bottom: Val::Px(100.0),
            ..default()
        },
        MainText,
        Text::new(text),
    ));
}
