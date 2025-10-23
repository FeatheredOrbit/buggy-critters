use bevy::prelude::*;

mod entity;
mod scene;
mod food;

mod materials;

use entity::EntityPlugin;
use food::FoodPlugin;
use scene::SceneManagerPlugin;
use materials::MaterialLoaderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialLoaderPlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(EntityPlugin)
        .run();
}
