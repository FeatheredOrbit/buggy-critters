use bevy::prelude::*;

use crate::entity::components::{render_components::*, shared_components::*, moving_components::*};
use crate::entity::states::moving::moving_utils::*;

pub fn moving_new_state(mut query: Query<(Entity, &mut Transform, &FutureTransform, &mut CurrentlyRotating, &mut CurrentlyMoving, &PhysicalTraits ), (With<MovingNew>, With<EntityRoot>)>, mut commands: Commands, time: Res<Time>) {
    for (entity, mut transform, future_transform, mut currently_rotating, mut currently_moving, physical_traits) in &mut query {
        
        if currently_rotating.0 && rotate_towards_position(&mut transform, future_transform, physical_traits, &time) {
            currently_rotating.0 = false;
            currently_moving.0 = true;
        }

        if currently_moving.0 && move_towards_position(&mut transform, future_transform, physical_traits, &time) {
            currently_moving.0 = false;
        }

        if !currently_rotating.0 && !currently_moving.0 {

            currently_rotating.0 = true;
            currently_moving.0 = false;

            commands.entity(entity).remove::<MovingNew>().insert(Idle);
        }

    }
}