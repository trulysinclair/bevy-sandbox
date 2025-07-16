use crate::power::power_consumer::PowerConsumer;
use crate::power::power_source::PowerSource;
use crate::wire_system::{ConnectionPoint, Wire};
use bevy::prelude::*;
use std::collections::HashSet;

pub struct PowerPlugin;

impl Plugin for PowerPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, setup)
            .add_systems(Update, power_propagation_system);
    }
}

// fn setup(mut commands: Commands) {}

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
