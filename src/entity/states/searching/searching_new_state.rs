use bevy::prelude::*;

use crate::entity::components::{shared_components::*, render_components::*};

pub fn searching_new_state(mut query: Query<Entity, (With<SearchingNew>, With<EntityRoot>)>) {
    for entity in query.iter_mut() {
        
    }
}