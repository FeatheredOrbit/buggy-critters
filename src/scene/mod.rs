use bevy::prelude::*;

mod scene_init;
pub mod components;

use scene_init::*;

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}