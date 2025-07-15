use crate::grid::{grid_to_world, GridPosition};
use bevy::prelude::*;
use std::collections::HashSet;

pub struct WireSystemPlugin;

impl Plugin for WireSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (power_propagation_system, wire_visual_system));
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

#[derive(Resource, Default)]
pub struct WireState {
    pub selected_connection: Option<Entity>,
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
        .filter_map(|(entity, source)| {
            if source.powered {
                Some(entity)
            } else {
                None
            }
        })
        .collect();

    // Propagate power through wire network
    let mut powered_entities = HashSet::new();
    for source_entity in powered_sources {
        powered_entities.insert(source_entity);
        propagate_power_from(source_entity, &wires, &connection_points, &mut powered_entities);
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
            let wire_material = materials.add(ColorMaterial::from_color(Color::srgb(1.0, 0.8, 0.0))); // Yellow/gold wire
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