use bevy::prelude::*;

use crate::{constants::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH}, bug_entity::components::{attribute_components::PhysicalTraits, render_components::*, shared_components::{States, *}}, food::fruit_entity::components::FruitEntityRoot, resources::FruitGrid};
use crate::bug_entity::states::searching::searching_utils::*;

pub fn searching_food_state(
    mut query: Query<(Entity, &Transform, &mut FutureTransform, &PhysicalTraits, &mut RngComponent), 
        (With<BugEntityRoot>, With<Searching>, With<SearchingFood>)>, 
    fruit_query: Query<&Transform, With<FruitEntityRoot>>, 
    fruit_grid: Res<FruitGrid>,
    mut commands: Commands
) {

    for (entity, transform, mut future_transform, physical_traits, mut rng) in &mut query {

        let cell_x = (transform.translation.x / GRID_CELL_WIDTH).floor() as i32;
        let cell_y = (transform.translation.y / GRID_CELL_HEIGHT).floor() as i32;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(fruits) = fruit_grid.0.get(&(cell_x + dx, cell_y + dy)) {
                    
                    if search_position_food(transform, &mut future_transform, physical_traits, fruits, fruit_query, &mut rng) {
                        commands.entity(entity).insert(StateChangeRequired(States::MovingFood));
                    }

                }
            }
        }

    }
}