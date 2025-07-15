use crate::grid;
use crate::grid::{GridPosition, Material2dHandle};
use crate::items::generator::Generator;
use crate::items::light::Light;
use crate::wire_system::ConnectionPoint;
use bevy::color::palettes::basic::GRAY;
use bevy::color::palettes::css::BROWN;
use bevy::prelude::*;

pub struct PowerPolePlugin;

#[derive(Component)]
pub struct PowerPole;

impl Plugin for PowerPolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        // Removed old power_propagation_system - now handled by WireSystemPlugin
    }
}

fn setup(mut commands: Commands) {}

// Old power system - replaced by wire-based power system
// pub fn power_propagation_system(
//     generator_query: Query<(&GridPosition, &Generator), With<Generator>>,
//     pole_query: Query<&GridPosition, With<PowerPole>>,
//     mut light_query: Query<(&GridPosition, &mut Light, &Material2dHandle), With<Light>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) { ... }

pub fn spawn_power_pole(
    commands: &mut Commands,
    pos: GridPosition,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let power_pole_size = 5.0;

    commands
        .spawn((
            Name::new("PowerPole"),
            PowerPole,
            Mesh2d(meshes.add(Circle::new(power_pole_size))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(BROWN))),
            Transform::from_translation(grid::grid_to_world(pos) + Vec3::Z),
            pos,
            ConnectionPoint::new(4), // Power poles support up to 4 connections
        ))
        .id()
}
