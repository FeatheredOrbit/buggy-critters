use bevy::prelude::*;

use crate::{constants::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH}, food::fruit_entity::components::FruitEntityRoot, resources::{FruitGrid, LargestEntitySight}};

pub fn update_food_grid(
    query: Query<(Entity, &Transform), With<FruitEntityRoot>>, 
    mut grid: ResMut<FruitGrid>,
    largest_sight: Res<LargestEntitySight>
) {
    grid.0.clear();

    for (entity, transform) in &query {

        let cell_x: i32;
        let cell_y: i32;

        if let Some(sight_radius) = largest_sight.radius {
            let cell_size = sight_radius.ceil() * 2.0 / 3.0; // Diameter * 1/3
            cell_x = (transform.translation.x / cell_size).floor() as i32; 
            cell_y = (transform.translation.y / cell_size).floor() as i32; 
        }
        else {
            cell_x = (transform.translation.x / GRID_CELL_WIDTH).floor() as i32;
            cell_y = (transform.translation.y / GRID_CELL_HEIGHT).floor() as i32;
        }

        grid.0.entry((cell_x, cell_y)).or_default().push((entity, transform.clone()));
    }
}