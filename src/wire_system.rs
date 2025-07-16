use crate::build_tool::BuildTool;
use crate::grid::{grid_to_world, GridPosition};
use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;
use std::collections::HashSet;

pub struct WireSystemPlugin;

impl Plugin for WireSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                power_propagation_system,
                wire_visual_system,
                wire_preview_system,
                cleanup_orphaned_wires,
            ),
        );
    }
}

#[derive(Component)]
pub struct ConnectionPoint {
    pub max_connections: u8,
    pub connections: HashSet<Entity>,
}

impl ConnectionPoint {
    pub fn new(max_connections: u8) -> Self {
        Self {
            max_connections,
            connections: HashSet::new(),
        }
    }

    pub fn can_connect(&self) -> bool {
        self.connections.len() < self.max_connections as usize
    }

    pub fn add_connection(&mut self, wire_entity: Entity) -> bool {
        if self.can_connect() {
            self.connections.insert(wire_entity);
            true
        } else {
            false
        }
    }

    pub fn remove_connection(&mut self, wire_entity: Entity) {
        self.connections.remove(&wire_entity);
    }
}

#[derive(Component)]
pub struct PowerSource {
    pub powered: bool,
}

impl Default for PowerSource {
    fn default() -> Self {
        Self { powered: true }
    }
}

#[derive(Component)]
pub struct PowerConsumer {
    pub powered: bool,
}

impl Default for PowerConsumer {
    fn default() -> Self {
        Self { powered: false }
    }
}

#[derive(Component)]
pub struct Wire {
    pub from: Entity,
    pub to: Entity,
}

#[derive(Component)]
pub struct WireVisual;

#[derive(Component)]
pub struct WirePreview;

#[derive(Resource, Default)]
pub struct WireState {
    pub selected_connection: Option<Entity>,
    pub selected_position: Option<Vec3>,
    pub preview_entity: Option<Entity>,
}

fn power_propagation_system(
    mut consumers: Query<(Entity, &mut PowerConsumer)>,
    sources: Query<(Entity, &PowerSource)>,
    wires: Query<&Wire>,
    connection_points: Query<&ConnectionPoint>,
) {
    // Reset all consumers to unpowered
    for (_, mut consumer) in consumers.iter_mut() {
        consumer.powered = false;
    }

    // Find all powered sources
    let powered_sources: Vec<Entity> = sources
        .iter()
        .filter_map(
            |(entity, source)| {
                if source.powered { Some(entity) } else { None }
            },
        )
        .collect();

    // Propagate power through wire network
    let mut powered_entities = HashSet::new();
    for source_entity in powered_sources {
        powered_entities.insert(source_entity);
        propagate_power_from(
            source_entity,
            &wires,
            &connection_points,
            &mut powered_entities,
        );
    }

    // Update consumer power state
    for (entity, mut consumer) in consumers.iter_mut() {
        if powered_entities.contains(&entity) {
            consumer.powered = true;
        }
    }
}

fn propagate_power_from(
    from_entity: Entity,
    wires: &Query<&Wire>,
    connection_points: &Query<&ConnectionPoint>,
    powered_entities: &mut HashSet<Entity>,
) {
    if let Ok(connection_point) = connection_points.get(from_entity) {
        for &wire_entity in &connection_point.connections {
            if let Ok(wire) = wires.get(wire_entity) {
                let target = if wire.from == from_entity {
                    wire.to
                } else {
                    wire.from
                };

                if !powered_entities.contains(&target) {
                    powered_entities.insert(target);
                    propagate_power_from(target, wires, connection_points, powered_entities);
                }
            }
        }
    }
}

fn wire_visual_system(
    mut commands: Commands,
    new_wires: Query<(Entity, &Wire), (With<Wire>, Without<WireVisual>)>,
    positions: Query<&GridPosition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (wire_entity, wire) in new_wires.iter() {
        // Get positions of connected entities
        if let (Ok(from_pos), Ok(to_pos)) = (positions.get(wire.from), positions.get(wire.to)) {
            let from_world = grid_to_world(*from_pos);
            let to_world = grid_to_world(*to_pos);

            // Calculate wire position and rotation
            let wire_center = (from_world + to_world) / 2.0;
            let direction = to_world - from_world;
            let length = direction.length();
            let angle = direction.y.atan2(direction.x);

            // Create wire visual
            let wire_material =
                materials.add(ColorMaterial::from_color(Color::srgb(1.0, 0.8, 0.0))); // Yellow/gold wire
            let wire_mesh = meshes.add(Rectangle::new(length, 2.0)); // 2 pixel thick wire

            commands.entity(wire_entity).insert((
                Mesh2d(wire_mesh),
                MeshMaterial2d(wire_material),
                Transform::from_translation(wire_center + Vec3::new(0.0, 0.0, 0.05))
                    .with_rotation(Quat::from_rotation_z(angle)),
                WireVisual,
            ));
        }
    }
}

fn wire_preview_system(
    mut commands: Commands,
    mut wire_state: ResMut<WireState>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    positions: Query<&GridPosition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    build_tool: Res<BuildTool>,
    existing_previews: Query<Entity, With<WirePreview>>,
    all_entities: Query<Entity>,
) {
    // Clean up any existing previews first - but validate they exist
    for preview_entity in existing_previews.iter() {
        if all_entities.get(preview_entity).is_ok() {
            commands.entity(preview_entity).despawn();
        }
    }
    wire_state.preview_entity = None;

    // Only show preview if we're using the wire tool
    if *build_tool != BuildTool::Wire {
        return;
    }

    let Ok((camera, camera_transform)) = camera.single() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    // If we have a selected connection, show preview
    if let Some(selected_entity) = wire_state.selected_connection {
        // Validate that the selected entity still exists
        if positions.get(selected_entity).is_err() {
            // Selected entity no longer exists, clear the selection
            wire_state.selected_connection = None;
            wire_state.selected_position = None;
            return;
        }

        if let Some(cursor_pos) = window.cursor_position() {
            let world_pos = camera
                .viewport_to_world_2d(camera_transform, cursor_pos)
                .unwrap_or(Vec2::ZERO);

            if let Ok(selected_grid_pos) = positions.get(selected_entity) {
                let selected_world_pos = grid_to_world(*selected_grid_pos);
                let direction = Vec2::new(world_pos.x, world_pos.y)
                    - Vec2::new(selected_world_pos.x, selected_world_pos.y);

                // Limit to 500px
                let limited_direction = if direction.length() > 500.0 {
                    direction.normalize() * 500.0
                } else {
                    direction
                };

                let end_pos =
                    Vec2::new(selected_world_pos.x, selected_world_pos.y) + limited_direction;
                let wire_center =
                    (Vec2::new(selected_world_pos.x, selected_world_pos.y) + end_pos) / 2.0;
                let length = limited_direction.length();
                let angle = limited_direction.y.atan2(limited_direction.x);

                // Create new preview with correct length
                let preview_material =
                    materials.add(ColorMaterial::from_color(Color::srgba(1.0, 1.0, 1.0, 0.5))); // Semi-transparent white
                let preview_mesh = meshes.add(Rectangle::new(length, 1.5)); // Slightly thinner than real wire

                let preview_entity = commands
                    .spawn((
                        Mesh2d(preview_mesh),
                        MeshMaterial2d(preview_material),
                        Transform::from_translation(wire_center.extend(0.06))
                            .with_rotation(Quat::from_rotation_z(angle)),
                        WirePreview,
                    ))
                    .id();

                wire_state.preview_entity = Some(preview_entity);
            }
        }
    }
}

fn cleanup_orphaned_wires(
    mut commands: Commands,
    wires: Query<(Entity, &Wire)>,
    mut connection_points: Query<&mut ConnectionPoint>,
    existing_entities: Query<Entity>,
) {
    for (wire_entity, wire) in wires.iter() {
        let from_exists = existing_entities.get(wire.from).is_ok();
        let to_exists = existing_entities.get(wire.to).is_ok();

        // If either endpoint is missing, clean up the wire
        if !from_exists || !to_exists {
            // Remove wire reference from the existing endpoint (if any)
            if from_exists {
                if let Ok(mut connection_point) = connection_points.get_mut(wire.from) {
                    connection_point.remove_connection(wire_entity);
                }
            }
            if to_exists {
                if let Ok(mut connection_point) = connection_points.get_mut(wire.to) {
                    connection_point.remove_connection(wire_entity);
                }
            }

            // Safely despawn the orphaned wire
            if existing_entities.get(wire_entity).is_ok() {
                commands.entity(wire_entity).despawn();
                println!("Cleaned up orphaned wire: {:?}", wire_entity);
            }
        }
    }
}
