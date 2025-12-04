use bevy::prelude::*;

pub mod components;
mod init;
mod fruit_updates;
mod render;
mod utils;

use init::*;
use fruit_updates::*;
use render::*;
use utils::*;

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_systems(Startup, spawn);

        app.add_systems(PreUpdate, update_food_grid);

        app.add_systems(Update, update_render);

        app.add_systems(Update, handle_being_eaten);

    }
}