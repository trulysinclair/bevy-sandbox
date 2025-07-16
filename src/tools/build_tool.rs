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
    Wire,
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
    mut text_spans: Query<&mut TextSpan>,
    main_text_entities: Query<&Children, With<MainText>>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        *build_tool = BuildTool::Generator;
        update_tool_text("Generator", &mut text_spans, &main_text_entities);
    }

    if keys.just_pressed(KeyCode::Digit2) {
        *build_tool = BuildTool::PowerPole;
        update_tool_text("Power Pole", &mut text_spans, &main_text_entities);
    }

    if keys.just_pressed(KeyCode::Digit3) {
        *build_tool = BuildTool::Light;
        update_tool_text("Light", &mut text_spans, &main_text_entities);
    }

    if keys.just_pressed(KeyCode::Digit4) {
        *build_tool = BuildTool::Wire;
        update_tool_text("Wire", &mut text_spans, &main_text_entities);
    }
}

fn update_tool_text(
    tool_name: &str,
    text_spans: &mut Query<&mut TextSpan>,
    main_text_entities: &Query<&Children, With<MainText>>,
) {
    // Find the child TextSpan entities of MainText parents and update them
    for children in main_text_entities.iter() {
        for child in children.iter() {
            if let Ok(mut text_span) = text_spans.get_mut(child) {
                **text_span = tool_name.into();
            }
        }
    }
}
