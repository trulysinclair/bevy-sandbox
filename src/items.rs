pub mod generator;
pub mod light;
pub mod power_pole;

use crate::items::generator::GeneratorPlugin;
use crate::items::light::LightPlugin;
use crate::items::power_pole::PowerPolePlugin;
use bevy::prelude::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_plugins((
            GeneratorPlugin,
            PowerPolePlugin,
            LightPlugin,
        ));
    }
}

fn setup(mut commands: Commands) {}
