use bevy::prelude::*;

use crate::entity::components::{render_components::*, shared_components::{*, NextState, States}, moving_components::*};
use crate::entity::states::moving::moving_utils::*;

pub fn moving_food_state(mut query: Query<(&mut Transform, &FutureTransform, &mut CurrentlyRotating, &mut CurrentlyMoving, &PhysicalTraits, &MovementPattern, &mut NextState), (With<Moving>, With<MovingNew>, With<EntityRoot>)>, time: Res<Time>) {
    query.par_iter_mut().for_each(| (mut transform, future_transform, mut currently_rotating, mut currently_moving, physical_traits, movement_pattern, mut next_state) | {

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

            next_state.0 = States::Idle;
        }

    })
}