use bevy::prelude::*;

mod components;
mod fruit_init;
mod fruit_updates;
mod render;

use fruit_init::*;
use fruit_updates::*;
use render::*;

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_systems(Startup, spawn);

        app.add_systems(Update, update_entity_material);

        app.add_systems(Update, handle_being_eaten);

    }
}