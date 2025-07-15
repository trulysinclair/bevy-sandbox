use crate::grid;
use crate::grid::{GridPosition, Material2dHandle};
use bevy::color::palettes::basic::{GREEN, RED};
use bevy::prelude::*;

pub struct GeneratorPlugin;

#[derive(Component)]
pub struct Generator {
    pub(crate) fuel_amount: f32,
    pub(crate) output: f32,
    pub(crate) max_output: f32,
    pub(crate) is_active: bool,
    pub(crate) burn_timer: Timer,
}

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_power);
    }
}

pub fn tick_power(
    time: Res<Time>,
    mut generator: Query<(&mut Generator, &Material2dHandle), With<Generator>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut generator, material_handle) in generator.iter_mut() {
        let has_fuel = generator.fuel_amount > 0.0;
        let can_output = generator.output < generator.max_output;
        let out_of_fuel = generator.fuel_amount == 0.0;

        // if (can_output) {
        if (has_fuel) {
            generator.burn_timer.tick(time.delta());

            generator.is_active = true;

            if (generator.burn_timer.finished()) {
                generator.fuel_amount -= 1.0;
                println!("Fuel left: {}", generator.fuel_amount);
            }

            generator.output += 1.0;

            if let Some(material) = materials.get_mut(&material_handle.0) {
                material.color = Color::from(GREEN);
            }
        }

        if (out_of_fuel) {
            generator.is_active = false;

            if let Some(material) = materials.get_mut(&material_handle.0) {
                material.color = Color::from(RED);
            }

            println!("No fuel left!")
        }
        // } else {
        // generator.is_active = false;
        //
        // if let Some(material) = materials.get_mut(&material_handle.0) {
        //     material.color = Color::from(YELLOW);
        // }
        //
        // println!("Battery is full!");
        // println!("Delta: {:?}", time.delta());
        // }
    }
}

pub fn spawn_generator(
    commands: &mut Commands,
    pos: GridPosition,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let gen_material_handle = materials.add(ColorMaterial::from_color(RED));
    commands
        .spawn((
            Name::new("Generator"),
            Generator {
                is_active: false,
                fuel_amount: 5.0,
                output: 0.0,
                max_output: 20.0,
                burn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            },
            Mesh2d(meshes.add(Triangle2d::new(
                Vec2::Y * 15.0,
                Vec2::new(-15.0, -15.0),
                Vec2::new(15.0, -15.0),
            ))),
            Material2dHandle(gen_material_handle.clone()),
            MeshMaterial2d(gen_material_handle.clone()),
            Transform::from_translation(grid::grid_to_world(pos) + Vec3::Z), // Render above tile
            pos,
        ))
        .id()
}
