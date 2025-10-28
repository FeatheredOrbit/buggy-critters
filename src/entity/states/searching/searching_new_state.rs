use bevy::prelude::*;

use crate::entity::components::{shared_components::{*, NextState, States}, render_components::*};
use crate::entity::states::searching::searching_utils::*;

pub fn searching_new_state(mut query: Query<(&Transform, &mut FutureTransform, &PhysicalTraits, &mut NextState), (With<Searching>, With<SearchingNew>, With<EntityRoot>)>) {
    query.par_iter_mut().for_each(| (transform, mut future_transform, physical_traits, mut next_state) | {
        if search_position_random(transform, &mut future_transform, physical_traits) {
            next_state.0 = States::MovingNew;
        }

    });
}