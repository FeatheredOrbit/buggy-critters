use bevy::prelude::*;

mod entity;
mod scene_manager;

use entity::EntityPlugin;
use scene_manager::SceneManagerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(EntityPlugin)
        .run();
}
