use bevy::prelude::*;

use crate::entity::components::{attribute_components::PhysicalTraits, shared_components::*};

const ROTATION_MULTIPLIER: f32 = 2.5;

pub fn rotate_towards_position_smooth(transform: &mut Transform, future_transform: &FutureTransform, physical_traits: &PhysicalTraits, time: &Time) -> bool {

    let current_rotation = transform.rotation;

    let rotation_step = physical_traits.speed * time.delta_secs() / ROTATION_MULTIPLIER;

    transform.rotation = current_rotation.slerp(future_transform.angle, rotation_step);

    let angle_diff = current_rotation.angle_between(future_transform.angle);

    if angle_diff <= 0.15 {

        transform.rotation = future_transform.angle;

        return true;
    };

    return false;
}

pub fn move_towards_position_smooth(transform: &mut Transform, future_transform: &FutureTransform, physical_traits: &PhysicalTraits, time: &Time) -> bool {
    
    let current_position = transform.translation;

    let step = physical_traits.speed * time.delta_secs();

    transform.translation = current_position.lerp(future_transform.position, step);

    let position_diff = current_position.distance(future_transform.position);

    if position_diff <= 1.0 {

        transform.translation = future_transform.position;

        return true;
    };

    return false;
}