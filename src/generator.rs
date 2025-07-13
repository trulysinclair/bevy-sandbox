use bevy::color::palettes::basic::RED;
use bevy::color::palettes::css::{GREEN, YELLOW};
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

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, (keyboard_input, tick_power));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material_handle = materials.add(ColorMaterial::from_color(RED));
    let burn_timer = Timer::from_seconds(1.0, TimerMode::Repeating);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        Material2dHandle(material_handle.clone()),
        MeshMaterial2d(material_handle),
        Transform::from_xyz(0.0, 50.0, 0.0),
        Generator {
            is_active: false,
            fuel_amount: 100.0,
            output: 2.0,
            max_output: 20.0,
            burn_timer,
        },
        Name::new("Generator"),
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
