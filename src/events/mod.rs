use bevy::{ecs::relationship::RelationshipSourceCollection, prelude::*};

use crate::{bug_entity::components::{attribute_components::PhysicalTraits, render_components::BugEntityRoot}, events::labels::{LargestSightUpdateType, UpdateLargestSight}, resources::LargestEntitySight};

pub mod labels;

pub struct SetupEventsPlugin;
impl Plugin for SetupEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_largest_sight_event);
    }
}

pub fn update_largest_sight_event(
    event: On<UpdateLargestSight>,
    query: Query<(Entity, &PhysicalTraits), With<BugEntityRoot>>,
    mut largest_entity_sight: ResMut<LargestEntitySight>
) {
    match event.0 {
        LargestSightUpdateType::SpawnInit => {
            let mut new_largest_sight = 0.0;
            let mut new_entity = Entity::new();

            for (entity, traits) in &query {
                if traits.sight > new_largest_sight {
                    new_largest_sight = traits.sight;
                    new_entity = entity;
                }
            }

            largest_entity_sight.radius = Some(new_largest_sight); 
            largest_entity_sight.entity = Some(new_entity);
        },

        LargestSightUpdateType::EntityDied => {
            let mut new_largest_sight = 0.0;
            let mut new_entity = Entity::new();

            for (entity, traits) in &query {
                if traits.sight > new_largest_sight {
                    new_largest_sight = traits.sight;
                    new_entity = entity;
                }
            }

            largest_entity_sight.radius = Some(new_largest_sight); 
            largest_entity_sight.entity = Some(new_entity);
        },


        LargestSightUpdateType::EntityBorn(entity) => {
            if let (Ok((entity, traits)), Some(largest_sight_radius)) = (query.get(entity), largest_entity_sight.radius.as_mut()) {
                if traits.sight > *largest_sight_radius {
                    *largest_sight_radius = traits.sight;
                    largest_entity_sight.entity = Some(entity);
                }
            }
        }
    }
}