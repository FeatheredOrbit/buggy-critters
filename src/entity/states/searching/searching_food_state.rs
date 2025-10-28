use bevy::prelude::*;

use crate::entity::components::{shared_components::*, render_components::*};
use crate::entity::states::searching::searching_utils::*;

pub fn searching_food_state(mut query: Query<(Entity, &Transform, &mut FutureTransform, &PhysicalTraits), (With<Searching>, With<SearchingNew>, With<EntityRoot>)>, mut commands: Commands) {
    for (entity, transform, mut future_transform, physical_traits) in &mut query {
        if search_position_random(transform, &mut future_transform, physical_traits) {
            commands.entity(entity).remove::<Searching>().remove::<SearchingNew>().insert((Moving, MovingNew));
        }

    }
}