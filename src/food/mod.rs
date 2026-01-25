use bevy::prelude::*;

pub mod fruit_entity;

use fruit_entity::FruitPlugin;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_plugins(FruitPlugin);

    }
}