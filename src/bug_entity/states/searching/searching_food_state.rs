use bevy::prelude::*;

use crate::{bug_entity::{components::{attribute_components::PhysicalTraits, render_components::*, shared_components::{States, *}}, utils::calculate_grid_cell_position}, resources::{FruitGrid, LargestEntitySight}};
use crate::bug_entity::states::searching::searching_utils::*;

pub fn searching_food_state(
    mut query: Query<(Entity, &Transform, &mut FutureTransform, &PhysicalTraits, &mut RngComponent), 
        (With<BugEntityRoot>, With<Searching>, With<SearchingFood>, Without<Dead>)>,
    fruit_grid: Res<FruitGrid>,
    largest_entity_sight: Res<LargestEntitySight>,
    mut commands: Commands
) {

    for (entity, transform, mut future_transform, physical_traits, mut rng) in &mut query {

        let cell_position = calculate_grid_cell_position(transform, &largest_entity_sight);
        let mut found = false;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(fruits) = fruit_grid.0.get(&(cell_position.0 + dx, cell_position.1 + dy)) {

                    if search_position_food(transform, &mut future_transform, physical_traits, fruits, &mut rng) {
                        commands.entity(entity).insert(StateChangeRequired(States::MovingFood));
                        found = true;
                        break;
                    }

                }
            }
            if found { break; }
        }

        if !found {
            if search_position_random(transform, &mut future_transform, physical_traits, &mut rng) {
                commands.entity(entity).insert(StateChangeRequired(States::MovingFood));
            }
        }

    }
}