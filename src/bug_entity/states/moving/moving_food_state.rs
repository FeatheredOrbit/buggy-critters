use bevy::prelude::*;

use crate::{constants::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH}, bug_entity::components::{moving_components::*, shared_components::{States, *}, attribute_components::*, render_components::BugEntityRoot}, food::fruit_entity::components::FruitEntityRoot, resources::FruitGrid};
use crate::bug_entity::states::moving::moving_utils::*;

pub fn moving_food_state(
    mut query: Query<(Entity, &mut Transform, &FutureTransform, &mut CurrentlyRotating, &mut CurrentlyMoving, &PhysicalTraits, &MovementPattern),
        (With<BugEntityRoot>, With<Moving>, With<MovingFood>, Without<FruitEntityRoot>)>,
    fruit_query: Query<&Transform, (With<FruitEntityRoot>, Without<BugEntityRoot>)>,
    fruit_grid: Res<FruitGrid>,
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

            if is_food_nearby(&transform, physical_traits, fruit_query, &fruit_grid) {
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
    fruit_query: Query<&Transform, (With<FruitEntityRoot>, Without<BugEntityRoot>)>,
    fruit_grid: &Res<FruitGrid>,
) -> bool {
    let cell_x = (transform.translation.x / GRID_CELL_WIDTH).floor() as i32;
    let cell_y = (transform.translation.y / GRID_CELL_HEIGHT).floor() as i32;

    for dx in -1..=1 {
        for dy in -1..=1 {

            if let Some(fruits) = fruit_grid.0.get(&(cell_x + dx, cell_y + dy)) {
                let current_position = transform.translation.xy();

                for fruit in fruits {
                    if let Ok(fruit_transform) = fruit_query.get(*fruit) {
                        let fruit_position = fruit_transform.translation.xy();
                        let distance = current_position.distance(fruit_position);

                        if distance <= traits.reach {
                            return true;
                        }
                    }
                }
            }

        }
    }

    return false;
}