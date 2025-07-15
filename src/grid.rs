use crate::build_tool::{BuildTool, TileContent};
use crate::items::generator::Generator;
use crate::items::light::Light;
use crate::items::power_pole::PowerPole;
use bevy::app::{App, Startup};
use bevy::asset::Assets;
use bevy::color::palettes::basic::{BLACK, BLUE, GRAY, RED, YELLOW};
use bevy::color::palettes::css::{BROWN, GREEN, GREY};
use bevy::color::palettes::tailwind::NEUTRAL_700;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (click_place_system));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Component)]
pub struct Material2dHandle(pub(crate) Handle<ColorMaterial>);

#[derive(Component)]
struct Hoverable;

#[derive(Component)]
struct Placed; // Marker for something placed on a tile

#[derive(Component)]
struct Tile {
    content: Option<Entity>, // child or placed thing
}

const TILE_SIZE: i32 = 16;
const GRID_SIZE: i32 = 32;
const SPACING: i32 = 2;

pub fn grid_to_world(pos: GridPosition) -> Vec3 {
    Vec3::new((pos.x * TILE_SIZE) as f32, (pos.y * TILE_SIZE) as f32, 0.0)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tiles_per_axis = GRID_SIZE / SPACING;
    let tile_half = tiles_per_axis / 2;

    for x in (0..GRID_SIZE) {
        for y in (0..GRID_SIZE) {
            let position = GridPosition {
                x: x - (tile_half * 2),
                y: y - (tile_half * 2),
            };

            let cell_material_handle = materials.add(ColorMaterial::from_color(NEUTRAL_700));
            let cell_mesh_handle = meshes.add(Rectangle::new(
                TILE_SIZE as f32 - 1.0,
                TILE_SIZE as f32 - 1.0,
            ));

            // Spawn tile
            commands.spawn((
                Mesh2d(cell_mesh_handle.clone()),
                Material2dHandle(cell_material_handle.clone()),
                MeshMaterial2d(cell_material_handle),
                Transform::from_translation(grid_to_world(position)),
                Hoverable,
                position,
                Tile { content: None },
            ));

            let border_material_handle = materials.add(ColorMaterial::from_color(BLACK));
            let tile_pos = grid_to_world(position);
            let half_tile = TILE_SIZE as f32 / 2.0;

            // Create 4 border lines (top, bottom, left, right)
            // Top border
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE as f32, 1.0))),
                MeshMaterial2d(border_material_handle.clone()),
                Transform::from_translation(tile_pos + Vec3::new(0.0, half_tile, 0.1)),
            ));

            // Bottom border
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE as f32, 1.0))),
                MeshMaterial2d(border_material_handle.clone()),
                Transform::from_translation(tile_pos + Vec3::new(0.0, -half_tile, 0.1)),
            ));

            // Left border
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(1.0, TILE_SIZE as f32))),
                MeshMaterial2d(border_material_handle.clone()),
                Transform::from_translation(tile_pos + Vec3::new(-half_tile, 0.0, 0.1)),
            ));

            // Right border
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(1.0, TILE_SIZE as f32))),
                MeshMaterial2d(border_material_handle),
                Transform::from_translation(tile_pos + Vec3::new(half_tile, 0.0, 0.1)),
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
    tool: Res<BuildTool>,
    mut commands: Commands,
    mut tiles: Query<
        (
            Entity,
            &GridPosition,
            &Material2dHandle,
            &mut Tile,
            Option<&TileContent>,
        ),
        With<Hoverable>,
    >,
    meshes: ResMut<Assets<Mesh>>,
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

    for (tile_entity, tile_pos, mat_handle, tile, existing) in tiles.iter_mut() {
        if *tile_pos != clicked_pos {
            continue;
        }

        if buttons.just_pressed(MouseButton::Right) && existing.is_none() {
            println!("Placing tile at {:?}", tile_pos);

            match *tool {
                BuildTool::Generator => {
                    let generator =
                        spawn_generator(&mut commands, *tile_pos, meshes, &mut materials);
                    commands.entity(tile_entity).insert(TileContent::Generator);
                    commands.entity(tile_entity).insert(Tile {
                        content: Some(generator),
                    });

                    if let Some(mat) = materials.get_mut(&mat_handle.0) {
                        mat.color = Color::from(GREEN);
                    }
                }
                BuildTool::PowerPole => {
                    let pole = spawn_power_pole(&mut commands, *tile_pos, meshes, &mut materials);
                    commands.entity(tile_entity).insert(TileContent::PowerPole);
                    commands.entity(tile_entity).insert(Tile {
                        content: Some(pole),
                    });
                    if let Some(mat) = materials.get_mut(&mat_handle.0) {
                        mat.color = Color::from(BLUE);
                    }
                }
                BuildTool::Light => {
                    let light = spawn_light(&mut commands, *tile_pos, meshes, &mut materials);
                    commands.entity(tile_entity).insert(TileContent::Light);
                    commands.entity(tile_entity).insert(Tile {
                        content: Some(light),
                    });
                    if let Some(mat) = materials.get_mut(&mat_handle.0) {
                        mat.color = Color::WHITE;
                    }
                }
            };
        }

        if buttons.just_pressed(MouseButton::Left) && existing.is_some() {
            println!("Removing tile at {:?}", tile_pos);
            if let Some(child_ent) = tile.content {
                commands.entity(child_ent).despawn();
            }
            commands
                .entity(tile_entity)
                .remove::<TileContent>()
                .insert(Tile { content: None });

            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                mat.color = Color::from(GRAY);
            }
        }

        break;
    }
}

fn spawn_generator(
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
            Transform::from_translation(grid_to_world(pos) + Vec3::Z), // Render above tile
            pos,
        ))
        .id()
}

fn spawn_power_pole(
    commands: &mut Commands,
    pos: GridPosition,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    commands
        .spawn((
            Name::new("PowerPole"),
            PowerPole,
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(BROWN))),
            Transform::from_translation(grid_to_world(pos) + Vec3::Z),
            pos,
        ))
        .id()
}

fn spawn_light(
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
            Transform::from_translation(grid_to_world(pos) + Vec3::Z),
            pos,
        ))
        .id()
}
