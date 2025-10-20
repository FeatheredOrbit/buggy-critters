use bevy::{prelude::*, sprite_render::Material2dPlugin};

pub mod entity_materials;

use entity_materials::*;

pub struct MaterialLoaderPlugin;

impl Plugin for MaterialLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FuzzMaterial>::default());

    }
}


