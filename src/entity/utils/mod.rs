use bevy::prelude::*;

use crate::entity::components::{shared_components::*, utils_components::*};

pub fn update_velocity(mut query: Query<(&Transform, &mut PreviousTransform, &mut Velocity), With<Moving>>, time: Res<Time>) {
    for (transform, mut previous_transform, mut velocity) in &mut query {

        let current_pos = transform.translation.xy();
        let previous_pos = previous_transform.0;

        velocity.0 = (current_pos - previous_pos) / time.delta_secs();

        previous_transform.0 = current_pos;
    }
}