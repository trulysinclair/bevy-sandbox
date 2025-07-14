use bevy::app::{App, Startup};
use bevy::asset::Assets;
use bevy::color::palettes::basic::YELLOW;
use bevy::color::palettes::css::GREEN;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::sprite::{Material2d, MeshMaterial2d};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (click_place_system));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridPosition {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Material2dHandle(Handle<ColorMaterial>);

#[derive(Component)]
struct Hoverable;

#[derive(Component)]
struct Placed; // Marker for something placed on a tile

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
                Material2dHandle(cell_material_handle.clone()),
                MeshMaterial2d(cell_material_handle),
                Transform::from_translation(grid_to_world(position)),
                Hoverable,
                position,
            ));
        }
    }
}

fn hover_mouse(
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut tiles: Query<(&GridPosition, &Material2dHandle), With<Hoverable>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let world_position = camera
        .viewport_to_world_2d(camera_transform, cursor_position)
        .unwrap_or(Vec2::ZERO);

    let grid_x = (world_position.x / TILE_SIZE as f32).round() as i32;
    let grid_y = (world_position.y / TILE_SIZE as f32).round() as i32;
    let hovered_tile = GridPosition {
        x: grid_x,
        y: grid_y,
    };

    for (position, handle) in tiles.iter_mut() {
        let color = if *position == hovered_tile {
            Color::from(YELLOW)
        } else {
            Color::from(GREEN)
        };

        if let Some(mat) = materials.get_mut(&handle.0) {
            mat.color = color;
        }
    }
}

fn click_place_system(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    mut tiles: Query<(Entity, &GridPosition, &Material2dHandle, Option<&Placed>), With<Hoverable>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok((camera, cam_transform)) = camera_q.single() else {
        return;
    };

    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let world_pos = match camera.viewport_to_world_2d(cam_transform, cursor_pos) {
        Ok(pos) => pos,
        Err(_) => return,
    };

    let grid_x = (world_pos.x / TILE_SIZE as f32).round() as i32;
    let grid_y = (world_pos.y / TILE_SIZE as f32).round() as i32;
    let clicked_pos = GridPosition {
        x: grid_x,
        y: grid_y,
    };

    for (entity, tile_pos, mat_handle, maybe_placed) in tiles.iter_mut() {
        if *tile_pos != clicked_pos {
            continue;
        }

        match (
            buttons.just_pressed(MouseButton::Right),
            buttons.just_pressed(MouseButton::Left),
        ) {
            (true, false) => {
                // Right-click to place something
                if maybe_placed.is_none() {
                    println!("Placing something on tile {:?}", tile_pos);
                    commands.entity(entity).insert(Placed);
                    if let Some(mat) = materials.get_mut(&mat_handle.0) {
                        mat.color = Color::from(YELLOW);
                    }
                }
            }
            (false, true) => {
                // Left-click to remove
                if maybe_placed.is_some() {
                    println!("Removing something from tile {:?}", tile_pos);
                    commands.entity(entity).remove::<Placed>();
                    if let Some(mat) = materials.get_mut(&mat_handle.0) {
                        mat.color = Color::from(GREEN);
                    }
                }
            }
            _ => {}
        }

        break; // Only one tile can be affected at a time
    }
}
