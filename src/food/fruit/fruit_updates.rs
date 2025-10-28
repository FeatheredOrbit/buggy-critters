use bevy::prelude::*;

use crate::food::fruit::components::*;


// Will handle jitteriness as [`NutritionalValue`] decreases
pub fn handle_being_eaten(mut query: Query<(Entity, &mut NutritionalValue), (With<Fruit>, With<BeingEaten>)>) {
    for (entity, mut nutritional_value) in &mut query {
        
    }
}