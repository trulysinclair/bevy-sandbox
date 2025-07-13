use bevy::color::palettes::basic::{GRAY, RED};
use bevy::color::palettes::css::{BROWN, GREEN, GREY, YELLOW};
use bevy::prelude::*;

pub struct GeneratorPlugin;

#[derive(Component)]
struct Generator {
    fuel_amount: f32,
    output: f32,
    max_output: f32,
    is_active: bool,
    burn_timer: Timer,
}

#[derive(Component)]
struct Material2dHandle(Handle<ColorMaterial>);

#[derive(Component)]
struct PowerPole;

#[derive(Component)]
struct Light {
    powered: bool,
}

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (keyboard_input, tick_power, power_propagation_system),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let gen_material_handle = materials.add(ColorMaterial::from_color(RED));
    let light_material_handle = materials.add(ColorMaterial::from_color(GREY));
    let burn_timer = Timer::from_seconds(1.0, TimerMode::Repeating);

    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::new(
            Vec2::Y * 15.0,
            Vec2::new(-15.0, -15.0),
            Vec2::new(15.0, -15.0),
        ))),
        Material2dHandle(gen_material_handle.clone()),
        MeshMaterial2d(gen_material_handle),
        Transform::from_xyz(0.0, 50.0, 0.0),
        Generator {
            is_active: false,
            fuel_amount: 5.0,
            output: 2.0,
            max_output: 20.0,
            burn_timer,
        },
        Name::new("Generator"),
    ));

    // Power pole
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(BROWN))),
        Transform::from_xyz(0.0, 0.0, 0.1),
        PowerPole,
        Name::new("PowerPole"),
    ));

    // Light
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
        Material2dHandle(light_material_handle.clone()),
        MeshMaterial2d(light_material_handle),
        Transform::from_xyz(0.0, -50.0, 0.2),
        Light { powered: false },
        Name::new("Light"),
    ));
}

fn tick_power(
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut generator: Query<(&mut Generator, &Material2dHandle), With<Generator>>,
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
    generator_query: Query<(&Transform, &Generator)>,
    pole_query: Query<&Transform, With<PowerPole>>,
    mut light_query: Query<(&Transform, &mut Light, &Material2dHandle), With<Light>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Find the first active generator
    for (gen_transform, generator) in generator_query.iter() {
        if generator.is_active {
            println!("Generator found!");
            // Check if a power pole is directly connected (within range)
            for pole_transform in pole_query.iter() {
                let distance_to_pole = gen_transform
                    .translation
                    .distance(pole_transform.translation);

                if distance_to_pole < 600.0 {
                    println!("Pole found!");

                    // If generator connects to pole, check for light below
                    for (light_transform, mut light, handle) in light_query.iter_mut() {
                        let distance_to_light = pole_transform
                            .translation
                            .distance(light_transform.translation);

                        println!("Distance to light: {}", distance_to_light);

                        if distance_to_light < 800.0 {
                            println!("Light found!");

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
