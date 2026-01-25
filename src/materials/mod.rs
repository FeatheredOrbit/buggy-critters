use bevy::prelude::*;

use crate::materials::renderer::RendererPlugin;

pub mod renderer;

pub struct MaterialLoaderPlugin;

impl Plugin for MaterialLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RendererPlugin);
    }
}


