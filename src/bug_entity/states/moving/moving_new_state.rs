use bevy::prelude::*;

use crate::bug_entity::components::{render_components::*, shared_components::{*, States}, moving_components::*, attribute_components::*};
use crate::bug_entity::states::moving::moving_utils::*;

pub fn moving_new_state(
    mut query: Query<(Entity, &mut Transform, &FutureTransform, &mut CurrentlyRotating, &mut CurrentlyMoving, &PhysicalTraits, &MovementPattern), 
        (With<BugEntityRoot>, With<Moving>, With<MovingNew>, Without<Dead>)>, 
    mut commands: Commands,
    time: Res<Time>
) {
    for (entity, mut transform, future_transform, mut currently_rotating, 
        mut currently_moving, physical_traits, movement_pattern
    ) in &mut query {

        let rotate_function: fn(&mut Transform, &FutureTransform, &PhysicalTraits, &Time) -> bool;
        let move_function: fn(&mut Transform, &FutureTransform, &PhysicalTraits, &Time) -> bool;

        match movement_pattern.0 {
            MovementPatterns::Smooth =>  { 
                rotate_function = rotate_towards_position_smooth;
                move_function = move_towards_position_smooth;
            },

            _ => {
                rotate_function = rotate_towards_position_smooth;
                move_function = move_towards_position_smooth;
            }
        }

        if currently_rotating.0 && rotate_function(&mut transform, future_transform, physical_traits, &time) {
            currently_rotating.0 = false;
            currently_moving.0 = true;
        }

        if currently_moving.0 && move_function(&mut transform, future_transform, physical_traits, &time) {
            currently_moving.0 = false;
        }

        if !currently_rotating.0 && !currently_moving.0 {

            currently_rotating.0 = true;
            currently_moving.0 = false;

            commands.entity(entity).insert(StateChangeRequired(States::Idle));
        }

    }
}