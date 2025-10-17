use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::entity::components::shared_components::{FutureTransform, PhysicalTraits};

pub fn search_position_random(transform: &Transform, future_transform: &mut FutureTransform, traits: &PhysicalTraits) -> bool {

    let current_position = Vec2::new(transform.translation.x, transform.translation.y);
    let range = traits.sight;

    let mut rng = thread_rng();

    let angle = rng.gen_range(0.0..=(PI * 2.0));

    let radius: f32 = rng.gen_range(0.0..=1.0);
    let radius = radius.sqrt() * range;

    let offset = Vec2::new(
        angle.cos() * radius,
        angle.sin() * radius
    );

    let next_position = current_position + offset;

    let direction = next_position - current_position;
    
    let next_angle = direction.y.atan2(direction.x);
    let next_quat = Quat::from_rotation_z(next_angle);

    future_transform.position = Vec3 { x: (next_position.x), y: (next_position.y), z: (0.0) };
    future_transform.angle = next_quat;

    return true;
    
    
}