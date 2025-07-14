use bevy::app::{App, Startup};
use bevy::asset::Assets;
use bevy::color::palettes::css::GREEN;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridPosition {
    x: i32,
    y: i32,
}

const TILE_SIZE: i32 = 16;
const GRID_SIZE: i32 = 32;
const SPACING: i32 = 2;

fn grid_to_world(pos: GridPosition) -> Vec3 {
    Vec3::new((pos.x * TILE_SIZE) as f32, (pos.y * TILE_SIZE) as f32, 0.0)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tiles_per_axis = GRID_SIZE / SPACING;
    let tile_half = tiles_per_axis / 2;

    for x in (0..GRID_SIZE).step_by(SPACING as usize) {
        for y in (0..GRID_SIZE).step_by(SPACING as usize) {
            let position = GridPosition {
                x: x - (tile_half * 2),
                y: y - (tile_half * 2),
            };

            let cell_material_handle = materials.add(ColorMaterial::from_color(GREEN));
            let cell_mesh_handle = meshes.add(Rectangle::new(TILE_SIZE as f32, TILE_SIZE as f32));

            commands.spawn((
                Mesh2d(cell_mesh_handle.clone()),
                MeshMaterial2d(cell_material_handle.clone()),
                Transform::from_translation(grid_to_world(position)),
                position,
            ));
        }
    }
}
