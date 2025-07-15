use crate::grid;
use crate::grid::{GridPosition, Material2dHandle};
use crate::wire_system::{ConnectionPoint, PowerConsumer};
use bevy::color::palettes::basic::GRAY;
use bevy::color::palettes::css::GREY;
use bevy::prelude::*;

#[derive(Component)]
pub struct Light {
    pub(crate) powered: bool,
}

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_light_visuals);
    }
}

fn setup(mut commands: Commands) {}

fn update_light_visuals(
    lights: Query<(&PowerConsumer, &Material2dHandle), With<Light>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (power_consumer, material_handle) in lights.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            if power_consumer.powered {
                material.color = Color::WHITE;
            } else {
                material.color = Color::from(GRAY);
            }
        }
    }
}

pub fn spawn_light(
    commands: &mut Commands,
    pos: GridPosition,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let light_material_handle = materials.add(ColorMaterial::from_color(GREY));
    let light_size = 10.0;

    commands
        .spawn((
            Name::new("Light"),
            Light { powered: false },
            Mesh2d(meshes.add(Rectangle::new(light_size, light_size))),
            Material2dHandle(light_material_handle.clone()),
            MeshMaterial2d(light_material_handle.clone()),
            Transform::from_translation(grid::grid_to_world(pos) + Vec3::Z),
            pos,
            ConnectionPoint::new(1), // Single connection point
            PowerConsumer::default(), // Lights are power consumers
        ))
        .id()
}
