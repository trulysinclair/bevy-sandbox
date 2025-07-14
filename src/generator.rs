use bevy::color::palettes::basic::{GRAY, RED};
use bevy::color::palettes::css::{BROWN, GREEN, GREY, YELLOW};
use bevy::prelude::*;
use crate::grid::{GridPosition, Material2dHandle};

pub struct GeneratorPlugin;

#[derive(Component)]
pub struct Generator {
    pub(crate) fuel_amount: f32,
    pub(crate) output: f32,
    pub(crate) max_output: f32,
    pub(crate) is_active: bool,
    pub(crate) burn_timer: Timer,
}

#[derive(Component)]
pub struct PowerPole;

#[derive(Component)]
pub struct Light {
    pub(crate) powered: bool,
}

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    keyboard_input,
                    tick_power,
                    power_propagation_system,
                ),
            );
    }
}

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let gen_material_handle = materials.add(ColorMaterial::from_color(RED));
//     let light_material_handle = materials.add(ColorMaterial::from_color(GREY));
//     let burn_timer = Timer::from_seconds(1.0, TimerMode::Repeating);
//     // Positions
//     let generator_pos = GridPosition { x: 1, y: 0 };
//     let pole_pos = GridPosition { x: 0, y: 0 };
//     let light_pos = GridPosition { x: -1, y: 0 };
//
//     commands.spawn((
//         Name::new("Generator"),
//         Generator {
//             is_active: false,
//             fuel_amount: 5.0,
//             output: 2.0,
//             max_output: 20.0,
//             burn_timer,
//         },
//         Mesh2d(meshes.add(Triangle2d::new(
//             Vec2::Y * 15.0,
//             Vec2::new(-15.0, -15.0),
//             Vec2::new(15.0, -15.0),
//         ))),
//         Material2dHandle(gen_material_handle.clone()),
//         MeshMaterial2d(gen_material_handle),
//         Transform::from_translation(grid_to_world(generator_pos)),
//         generator_pos,
//     ));
//
//     // Power pole
//     commands.spawn((
//         Mesh2d(meshes.add(Circle::new(10.0))),
//         MeshMaterial2d(materials.add(ColorMaterial::from_color(BROWN))),
//         Transform::from_translation(grid_to_world(pole_pos)),
//         PowerPole,
//         Name::new("PowerPole"),
//         pole_pos,
//     ));
//
//     // Light
//     commands.spawn((
//         Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
//         Material2dHandle(light_material_handle.clone()),
//         MeshMaterial2d(light_material_handle),
//         Transform::from_translation(grid_to_world(light_pos)),
//         Light { powered: false },
//         Name::new("Light"),
//         light_pos,
//     ));
// }

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

fn keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit.write(AppExit::Success);
    }
}

fn power_propagation_system(
    generator_query: Query<(&GridPosition, &Generator), With<Generator>>,
    pole_query: Query<&GridPosition, With<PowerPole>>,
    mut light_query: Query<(&GridPosition, &mut Light, &Material2dHandle), With<Light>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Find the first active generator
    for (gen_pos, generator) in generator_query.iter() {
        if generator.is_active {
            for pole_pos in pole_query.iter() {
                if gen_pos
                    == &(GridPosition {
                        x: pole_pos.x + 1,
                        y: pole_pos.y,
                    })
                {
                    // Generator is directly above pole
                    for (light_pos, mut light, handle) in light_query.iter_mut() {
                        if *pole_pos
                            == (GridPosition {
                                x: light_pos.x + 1,
                                y: light_pos.y,
                            })
                        {
                            // Pole is directly above light
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
