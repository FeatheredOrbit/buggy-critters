use bevy::prelude::*;

use crate::{constants::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH}, entity::components::{render_components::*, shared_components::{NextState, States, *}}, food::fruit::components::Fruit, resources::FruitGrid};
use crate::entity::states::searching::searching_utils::*;

pub fn searching_food_state(
    mut query: Query<(&Transform, &mut FutureTransform, &PhysicalTraits, &mut NextState), (With<EntityRoot>, With<SearchingFood>, With<Searching>)>, 
    fruit_query: Query<&Transform, With<Fruit>>, 
    fruit_grid: Res<FruitGrid>
) {

    query.par_iter_mut().for_each(| (transform, mut future_transform, physical_traits, mut next_state) | {

        let cell_x = (transform.translation.x / GRID_CELL_WIDTH).floor() as i32;
        let cell_y = (transform.translation.y / GRID_CELL_HEIGHT).floor() as i32;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(fruits) = fruit_grid.0.get(&(cell_x + dx, cell_y + dy)) {
                    
                    if search_position_food(transform, &mut future_transform, physical_traits, fruits, fruit_query) {
                        next_state.0 = States::MovingFood;
                    }

                }
            }
        }

    })
}