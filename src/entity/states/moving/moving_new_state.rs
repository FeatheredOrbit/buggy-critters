use bevy::prelude::*;

use crate::entity::components::{render_components::*, shared_components::*};

pub fn moving_new_state(mut query: Query<(Entity, &mut Transform, &FutureTransform), (With<MovingNew>, With<EntityRoot>)>, commands: Commands) {
    for (entity, mut transform, future_transform) in &mut query {
        
    }
}