use crate::grid;
use crate::grid::{GridPosition, Material2dHandle};
use bevy::color::palettes::css::GREY;
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

pub fn spawn_light(
    commands: &mut Commands,
    pos: GridPosition,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let light_material_handle = materials.add(ColorMaterial::from_color(GREY));

    commands
        .spawn((
            Name::new("Light"),
            Light { powered: false },
            Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
            Material2dHandle(light_material_handle.clone()),
            MeshMaterial2d(light_material_handle.clone()),
            Transform::from_translation(grid::grid_to_world(pos) + Vec3::Z),
            pos,
        ))
        .id()
}
