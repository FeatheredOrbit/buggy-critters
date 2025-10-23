use bevy::prelude::*;

mod fruit;

use fruit::FruitPlugin;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_plugins(FruitPlugin);

    }
}