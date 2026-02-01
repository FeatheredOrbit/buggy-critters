use core::f32;
use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

use crate::bug_entity::components::shared_components::RngComponent;
use crate::bug_entity::components::{attribute_components::PhysicalTraits, shared_components::FutureTransform}; 

use crate::food::fruit_entity::components::FruitEntityRoot;

pub fn search_position_random(
    transform: &Transform, 
    future_transform: &mut FutureTransform, 
    traits: &PhysicalTraits, 
    rng: &mut RngComponent
) -> bool {

    let current_position = Vec2::new(transform.translation.x, transform.translation.y);
    let range = traits.sight;

    let angle = rng.0.random_range(0.0..=(PI * 2.0));

    let radius: f32 = rng.0.random_range(0.0..=1.0);
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

pub fn search_position_food(
    transform: &Transform, 
    future_transform: &mut FutureTransform, 
    traits: &PhysicalTraits, 
    fruits: &Vec<Entity>, 
    fruit_query: Query<&Transform, With<FruitEntityRoot>>,
    rng: &mut RngComponent
) -> bool {

    let mut closest_distance = f32::INFINITY;
    let mut next_position = Vec2::ZERO;

    let current_position = transform.translation.xy();
    

    for fruit in fruits {

        if let Ok(fruit_transform) = fruit_query.get(*fruit) {

            let fruit_position = fruit_transform.translation.xy();

            let distance = current_position.distance(fruit_position);

            if distance < closest_distance {
                closest_distance = distance;
                next_position = fruit_position;
            }

        }

        if closest_distance <= traits.sight {

            let direction = next_position - current_position;

            let next_angle = direction.y.atan2(direction.x);
            let next_quat = Quat::from_rotation_z(next_angle);
            
            future_transform.position = Vec3 { x: (next_position.x), y: (next_position.y), z: (0.0) };
            future_transform.angle = next_quat;

            return true;
        }

        return search_position_random(transform, future_transform, traits, rng);

    }

    return search_position_random(transform, future_transform, traits, rng);
}