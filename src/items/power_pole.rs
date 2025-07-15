use crate::grid::{GridPosition, Material2dHandle};
use crate::items::generator::Generator;
use crate::items::light::Light;
use bevy::color::palettes::basic::GRAY;
use bevy::prelude::*;

pub struct PowerPolePlugin;

#[derive(Component)]
pub struct PowerPole;

impl Plugin for PowerPolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, power_propagation_system);
    }
}

fn setup(mut commands: Commands) {}

pub fn power_propagation_system(
    generator_query: Query<(&GridPosition, &Generator), With<Generator>>,
    pole_query: Query<&GridPosition, With<PowerPole>>,
    mut light_query: Query<(&GridPosition, &mut Light, &Material2dHandle), With<Light>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Find the first active generator
    for (gen_pos, generator) in generator_query.iter() {
        println!("Generator at {:?}", gen_pos);
        if generator.is_active {
            for pole_pos in pole_query.iter() {
                println!("Pole at {:?}", pole_pos);
                if gen_pos
                    == &(GridPosition {
                        x: pole_pos.x + 1,
                        y: pole_pos.y,
                    })
                {
                    // Generator is directly left to a pole
                    for (light_pos, mut light, handle) in light_query.iter_mut() {
                        println!("Light at {:?}", light_pos);
                        if *pole_pos
                            == (GridPosition {
                                x: light_pos.x + 1,
                                y: light_pos.y,
                            })
                        {
                            // Pole is directly right of a light
                            light.powered = true;
                            if let Some(mat) = materials.get_mut(&handle.0) {
                                mat.color = Color::WHITE;
                            }
                        }
                    }
                }
            }
        }
    }

    // Turn off unpowered lights
    for (_, mut light, handle) in light_query.iter_mut() {
        if !light.powered {
            if let Some(mat) = materials.get_mut(&handle.0) {
                mat.color = Color::from(GRAY);
            }
        }
        // Reset for next tick
        light.powered = false;
    }
}
