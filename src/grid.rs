use crate::build_tool::{BuildTool, TileContent};
use crate::items::{generator, light, power_pole};
use crate::wire_system::{ConnectionPoint, Wire, WireState};
use bevy::app::{App, Startup};
use bevy::asset::Assets;
use bevy::color::palettes::basic::{BLACK, WHITE};
use bevy::color::palettes::tailwind::NEUTRAL_700;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoverState>()
            .init_resource::<WireState>()
            .add_systems(Startup, (setup, setup_hover_borders))
            .add_systems(
                Update,
                (
                    click_place_system,
                    hover_mouse,
                    process_pending_wire_connections,
                ),
            );
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
struct HoverBorder; // Marker for hover border entities

#[derive(Resource)]
struct HoverBorderEntities {
    top: Entity,
    bottom: Entity,
    left: Entity,
    right: Entity,
}

#[derive(Resource, Default)]
struct HoverState {
    last_hovered: Option<GridPosition>,
}

#[derive(Component)]
struct PendingWireConnection {
    from: Entity,
    to: Entity,
}

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

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
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

fn setup_hover_borders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let border_material_handle = materials.add(ColorMaterial::from_color(WHITE));

    // Create 4 persistent border entities, initially hidden
    let top = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(TILE_SIZE as f32, 1.0))),
            MeshMaterial2d(border_material_handle.clone()),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
            Visibility::Hidden,
            HoverBorder,
        ))
        .id();

    let bottom = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(TILE_SIZE as f32, 1.0))),
            MeshMaterial2d(border_material_handle.clone()),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
            Visibility::Hidden,
            HoverBorder,
        ))
        .id();

    let left = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(1.0, TILE_SIZE as f32))),
            MeshMaterial2d(border_material_handle.clone()),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
            Visibility::Hidden,
            HoverBorder,
        ))
        .id();

    let right = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(1.0, TILE_SIZE as f32))),
            MeshMaterial2d(border_material_handle),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
            Visibility::Hidden,
            HoverBorder,
        ))
        .id();

    commands.insert_resource(HoverBorderEntities {
        top,
        bottom,
        left,
        right,
    });
}

fn hover_mouse(
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    tiles: Query<&GridPosition, With<Hoverable>>,
    hover_entities: Res<HoverBorderEntities>,
    mut hover_state: ResMut<HoverState>,
    mut transforms: Query<&mut Transform>,
    mut visibility: Query<&mut Visibility>,
) {
    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        // Hide borders when cursor is not in window only if we were previously showing something
        if hover_state.last_hovered.is_some() {
            hover_state.last_hovered = None;
            if let Ok(mut vis) = visibility.get_mut(hover_entities.top) {
                *vis = Visibility::Hidden;
            }
            if let Ok(mut vis) = visibility.get_mut(hover_entities.bottom) {
                *vis = Visibility::Hidden;
            }
            if let Ok(mut vis) = visibility.get_mut(hover_entities.left) {
                *vis = Visibility::Hidden;
            }
            if let Ok(mut vis) = visibility.get_mut(hover_entities.right) {
                *vis = Visibility::Hidden;
            }
        }
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

    // Early return if we're hovering the same tile as last frame
    if hover_state.last_hovered == Some(hovered_tile) {
        return;
    }

    // Check if we're hovering over a valid tile
    let is_hovering_valid_tile = tiles.iter().any(|pos| *pos == hovered_tile);

    if is_hovering_valid_tile {
        hover_state.last_hovered = Some(hovered_tile);
        let tile_pos = grid_to_world(hovered_tile);
        let half_tile = TILE_SIZE as f32 / 2.0;

        // Update positions and show borders
        if let Ok(mut transform) = transforms.get_mut(hover_entities.top) {
            transform.translation = tile_pos + Vec3::new(0.0, half_tile, 0.1);
        }
        if let Ok(mut vis) = visibility.get_mut(hover_entities.top) {
            *vis = Visibility::Visible;
        }

        if let Ok(mut transform) = transforms.get_mut(hover_entities.bottom) {
            transform.translation = tile_pos + Vec3::new(0.0, -half_tile, 0.1);
        }
        if let Ok(mut vis) = visibility.get_mut(hover_entities.bottom) {
            *vis = Visibility::Visible;
        }

        if let Ok(mut transform) = transforms.get_mut(hover_entities.left) {
            transform.translation = tile_pos + Vec3::new(-half_tile, 0.0, 0.1);
        }
        if let Ok(mut vis) = visibility.get_mut(hover_entities.left) {
            *vis = Visibility::Visible;
        }

        if let Ok(mut transform) = transforms.get_mut(hover_entities.right) {
            transform.translation = tile_pos + Vec3::new(half_tile, 0.0, 0.1);
        }
        if let Ok(mut vis) = visibility.get_mut(hover_entities.right) {
            *vis = Visibility::Visible;
        }
    } else {
        // Hide borders when not hovering over a valid tile
        hover_state.last_hovered = None;
        if let Ok(mut vis) = visibility.get_mut(hover_entities.top) {
            *vis = Visibility::Hidden;
        }
        if let Ok(mut vis) = visibility.get_mut(hover_entities.bottom) {
            *vis = Visibility::Hidden;
        }
        if let Ok(mut vis) = visibility.get_mut(hover_entities.left) {
            *vis = Visibility::Hidden;
        }
        if let Ok(mut vis) = visibility.get_mut(hover_entities.right) {
            *vis = Visibility::Hidden;
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
    mut wire_state: ResMut<WireState>,
    mut connection_points: Query<&mut ConnectionPoint>,
    items: Query<(Entity, &GridPosition), With<ConnectionPoint>>,
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

    for (tile_entity, tile_pos, _mat_handle, tile, existing) in tiles.iter_mut() {
        if *tile_pos != clicked_pos {
            continue;
        }

        if buttons.just_pressed(MouseButton::Right) {
            match *tool {
                BuildTool::Wire => {
                    // Wire placement logic
                    if let Some(item_entity) = find_item_at_position(*tile_pos, &items) {
                        // Existing item found - connect to it
                        handle_wire_placement(
                            item_entity,
                            *tile_pos,
                            &mut wire_state,
                            &mut connection_points,
                            &mut commands,
                        );
                    } else if wire_state.selected_connection.is_some() && existing.is_none() {
                        // Empty tile and we have a selected connection - spawn pole and connect
                        handle_wire_to_empty_tile(
                            tile_entity,
                            *tile_pos,
                            &mut wire_state,
                            &mut commands,
                            meshes,
                            &mut materials,
                        );
                    }
                }
                _ if existing.is_none() => {
                    // Regular item placement
                    match *tool {
                        BuildTool::Generator => {
                            let generator = generator::spawn_generator(
                                &mut commands,
                                *tile_pos,
                                meshes,
                                &mut materials,
                            );

                            commands.entity(tile_entity).insert(TileContent::Generator);
                            commands.entity(tile_entity).insert(Tile {
                                content: Some(generator),
                            });
                        }
                        BuildTool::PowerPole => {
                            let pole = power_pole::spawn_power_pole(
                                &mut commands,
                                *tile_pos,
                                meshes,
                                &mut materials,
                            );

                            commands.entity(tile_entity).insert(TileContent::PowerPole);
                            commands.entity(tile_entity).insert(Tile {
                                content: Some(pole),
                            });
                        }
                        BuildTool::Light => {
                            let light = light::spawn_light(
                                &mut commands,
                                *tile_pos,
                                meshes,
                                &mut materials,
                            );

                            commands.entity(tile_entity).insert(TileContent::Light);
                            commands.entity(tile_entity).insert(Tile {
                                content: Some(light),
                            });
                        }
                        BuildTool::Wire => {}
                    }
                }
                _ => {}
            }
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
        }

        break;
    }
}

fn find_item_at_position(
    pos: GridPosition,
    items: &Query<(Entity, &GridPosition), With<ConnectionPoint>>,
) -> Option<Entity> {
    for (entity, item_pos) in items.iter() {
        if *item_pos == pos {
            return Some(entity);
        }
    }
    None
}

fn handle_wire_placement(
    item_entity: Entity,
    pos: GridPosition,
    wire_state: &mut ResMut<WireState>,
    connection_points: &mut Query<&mut ConnectionPoint>,
    commands: &mut Commands,
) {
    if let Some(selected) = wire_state.selected_connection {
        // Second click - try to create wire
        if selected != item_entity {
            if can_connect(selected, item_entity, connection_points) {
                create_wire(selected, item_entity, connection_points, commands);
            }
        }
        // Clear selection and preview
        wire_state.selected_connection = None;
        wire_state.selected_position = None;
        wire_state.preview_entity = None; // Just clear the reference, preview system will handle cleanup
    } else {
        // First click - select connection point
        if let Ok(connection_point) = connection_points.get(item_entity) {
            if connection_point.can_connect() {
                wire_state.selected_connection = Some(item_entity);
                wire_state.selected_position = Some(grid_to_world(pos));
                println!("Selected connection point at {:?}", pos);
            }
        }
    }
}

fn can_connect(
    from: Entity,
    to: Entity,
    connection_points: &mut Query<&mut ConnectionPoint>,
) -> bool {
    if let (Ok(from_conn), Ok(to_conn)) = (connection_points.get(from), connection_points.get(to)) {
        from_conn.can_connect() && to_conn.can_connect()
    } else {
        false
    }
}

fn create_wire(
    from: Entity,
    to: Entity,
    connection_points: &mut Query<&mut ConnectionPoint>,
    commands: &mut Commands,
) {
    // Validate that both entities exist and have connection points
    let from_valid = connection_points.get(from).is_ok();
    let to_valid = connection_points.get(to).is_ok();

    if !from_valid || !to_valid {
        println!(
            "Cannot create wire: invalid entities (from: {}, to: {})",
            from_valid, to_valid
        );
        return;
    }

    let wire_entity = commands
        .spawn((
            Wire { from, to },
            Name::new("Wire"),
            // Visual representation will be added by wire_visual_system
        ))
        .id();

    // Add wire to connection points
    if let Ok(mut from_conn) = connection_points.get_mut(from) {
        from_conn.add_connection(wire_entity);
    }
    if let Ok(mut to_conn) = connection_points.get_mut(to) {
        to_conn.add_connection(wire_entity);
    }

    println!("Created wire between {:?} and {:?}", from, to);
}

fn handle_wire_to_empty_tile(
    tile_entity: Entity,
    pos: GridPosition,
    wire_state: &mut ResMut<WireState>,
    commands: &mut Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    if let Some(selected_entity) = wire_state.selected_connection {
        // Spawn a power pole at the empty tile
        let pole_entity = power_pole::spawn_power_pole(commands, pos, meshes, materials);

        // Update the tile to contain the pole
        commands.entity(tile_entity).insert(TileContent::PowerPole);
        commands.entity(tile_entity).insert(Tile {
            content: Some(pole_entity),
        });

        // Schedule wire creation for next frame when pole components are ready
        commands.spawn(PendingWireConnection {
            from: selected_entity,
            to: pole_entity,
        });

        // Clear selection and preview
        wire_state.selected_connection = None;
        wire_state.selected_position = None;
        wire_state.preview_entity = None; // Just clear the reference, preview system will handle cleanup

        println!(
            "Spawned power pole and scheduled wire connection at {:?}",
            pos
        );
    }
}

fn process_pending_wire_connections(
    mut commands: Commands,
    mut connection_points: Query<&mut ConnectionPoint>,
    pending_connections: Query<(Entity, &PendingWireConnection)>,
    existing_entities: Query<Entity>,
) {
    for (pending_entity, pending) in pending_connections.iter() {
        // Check if both entities still exist
        let from_exists = existing_entities.get(pending.from).is_ok();
        let to_exists = existing_entities.get(pending.to).is_ok();

        if !from_exists || !to_exists {
            // One or both entities no longer exist, cancel the pending connection
            println!(
                "Cancelling pending wire connection - entity no longer exists (from: {}, to: {})",
                from_exists, to_exists
            );
            commands.entity(pending_entity).despawn();
            continue;
        }

        // Check if both entities now have ConnectionPoint components
        if connection_points.get(pending.from).is_ok() && connection_points.get(pending.to).is_ok()
        {
            // Create the wire connection
            if can_connect(pending.from, pending.to, &mut connection_points) {
                create_wire(
                    pending.from,
                    pending.to,
                    &mut connection_points,
                    &mut commands,
                );
                println!(
                    "Successfully created deferred wire connection between {:?} and {:?}",
                    pending.from, pending.to
                );
            } else {
                println!("Failed to connect - connection points at capacity");
            }

            // Remove the pending connection
            commands.entity(pending_entity).despawn();
        }
    }
}
