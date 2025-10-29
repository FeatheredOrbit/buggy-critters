use std::collections::HashMap;

use bevy::prelude::*;

mod entity;
mod scene;
mod food;
mod constants;

mod materials;
mod resources;

use entity::EntityPlugin;
use food::FoodPlugin;
use scene::SceneManagerPlugin;
use materials::MaterialLoaderPlugin;

use resources::{EntityGrid, FruitGrid};

fn main() {
    App::new()
        .insert_resource(EntityGrid(HashMap::new()))
        .insert_resource(FruitGrid(HashMap::new()))
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialLoaderPlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(EntityPlugin)
        .run();
}
