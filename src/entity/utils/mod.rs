use bevy::prelude::*;

use crate::constants::*;

use crate::entity::components::render_components::EntityRoot;
use crate::{entity::components::{shared_components::*, utils_components::*}, resources::EntityGrid};

pub fn update_velocity(mut query: Query<(&Transform, &mut PreviousTransform, &mut Velocity), With<Moving>>, time: Res<Time>) {
    for (transform, mut previous_transform, mut velocity) in &mut query {

        let current_pos = transform.translation.xy();
        let previous_pos = previous_transform.0;

        velocity.0 = (current_pos - previous_pos) / time.delta_secs();

        previous_transform.0 = current_pos;
    }
}

pub fn update_entity_grid(query: Query<(Entity, &Transform), With<EntityRoot>>, mut grid: ResMut<EntityGrid>) {
    grid.0.clear();

    for (entity, transform) in &query {

        let cell_x = (transform.translation.x / GRID_CELL_WIDTH).floor() as i32;
        let cell_y = (transform.translation.y / GRID_CELL_HEIGHT).floor() as i32;

        grid.0.entry((cell_x, cell_y)).or_default().push(entity);

    }
}