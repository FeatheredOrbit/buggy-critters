use bevy::prelude::*;

use crate::food::fruit::components::*;

pub fn handle_being_eaten(mut query: Query<(Entity, &mut NutritionalValue), (With<Fruit>, With<BeingEaten>)>) {
    for (entity, mut nutritional_value) in &mut query {
        
    }
}