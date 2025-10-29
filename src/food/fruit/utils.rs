use bevy::prelude::*;

use crate::{constants::{GRID_CELL_WIDTH, GRID_CELL_HEIGHT}, food::fruit::components::Fruit, resources::FruitGrid};

pub fn update_food_grid(query: Query<(Entity, &Transform), With<Fruit>>, mut grid: ResMut<FruitGrid>) {
    for (entity, transform) in &query {

        let cell_x = (transform.translation.x / GRID_CELL_WIDTH).floor() as i32;
        let cell_y = (transform.translation.y / GRID_CELL_HEIGHT).floor() as i32;

        grid.0.entry((cell_x, cell_y)).or_default().push(entity);
    }
}