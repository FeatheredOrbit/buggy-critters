use bevy::prelude::*;

use crate::bug_entity::components::{attribute_components::PhysicalTraits, render_components::*, shared_components::{States, *}};
use crate::bug_entity::states::searching::searching_utils::*;

pub fn searching_new_state(
    mut query: Query<(Entity, &Transform, &mut FutureTransform, &PhysicalTraits, &mut RngComponent), 
        (With<BugEntityRoot>, With<Searching>, With<SearchingNew>)>,
    mut commands: Commands
) {
    for (entity, transform, mut future_transform, physical_traits, mut rng) in &mut query {
        if search_position_random(transform, &mut future_transform, physical_traits, &mut rng) {
            commands.entity(entity).insert(StateChangeRequired(States::MovingNew));
        }

    }
}