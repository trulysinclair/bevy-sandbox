use crate::grid;
use crate::grid::GridPosition;
use crate::wire_system::ConnectionPoint;
use bevy::color::palettes::css::BROWN;
use bevy::prelude::*;

pub struct PowerPolePlugin;

#[derive(Component)]
pub struct PowerPole;

impl Plugin for PowerPolePlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Startup, setup);
    }
}

// fn setup(mut commands: Commands) {}

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
            // Power poles support up to 4 connections
            ConnectionPoint::new(4),
        ))
        .id()
}
