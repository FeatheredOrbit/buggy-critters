use bevy::prelude::*;

mod entity;
mod scene_manager;

mod materials;

use entity::EntityPlugin;
use scene_manager::SceneManagerPlugin;
use materials::MaterialLoaderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialLoaderPlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(EntityPlugin)
        .run();
}
