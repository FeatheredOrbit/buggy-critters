use bevy::prelude::*;

use crate::{bug_entity::{components::{attribute_components::*, moving_components::*, render_components::BugEntityRoot, shared_components::{States, *}}, utils::calculate_grid_cell_position}, resources::{FruitGrid, LargestEntitySight}};
use crate::bug_entity::states::moving::moving_utils::*;

pub fn moving_food_state(
    mut query: Query<(Entity, &mut Transform, &FutureTransform, &mut CurrentlyRotating, &mut CurrentlyMoving, &PhysicalTraits, &MovementPattern),
        (With<BugEntityRoot>, With<Moving>, With<MovingFood>)>,
    fruit_grid: Res<FruitGrid>,
    largest_entity_sight: Res<LargestEntitySight>,
    mut commands: Commands,
    time: Res<Time>
) {
    for (
            entity, mut transform, future_transform,
            mut currently_rotating, mut currently_moving, physical_traits, 
            movement_pattern
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

            if is_food_nearby(&transform, physical_traits, &fruit_grid, &largest_entity_sight) {
                commands.entity(entity).insert(StateChangeRequired(States::Idle));
            } else {
                commands.entity(entity).insert(StateChangeRequired(States::SearchingFood));
            };
        }

    }
}

fn is_food_nearby(
    transform: &Transform,
    traits: &PhysicalTraits,
    fruit_grid: &Res<FruitGrid>,
    largest_entity_sight: &Res<LargestEntitySight>
) -> bool {
    let cell_position = calculate_grid_cell_position(transform, largest_entity_sight);

    for dx in -1..=1 {
        for dy in -1..=1 {

            if let Some(fruits) = fruit_grid.0.get(&(cell_position.0 + dx, cell_position.1 + dy)) {
                let current_position = transform.translation.xy();

                for fruit in fruits {
                    
                    let fruit_position = fruit.1.translation.xy();
                    let distance = current_position.distance(fruit_position);

                    if distance <= traits.reach {
                        return true;
                    }
                    
                }
            }

        }
    }

    return false;
}