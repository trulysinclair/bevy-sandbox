use bevy::prelude::*;

#[derive(Component)]
pub struct Light {
    pub(crate) powered: bool,
}

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {}
