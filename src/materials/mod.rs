use bevy::prelude::*;

use crate::materials::{death_explosion_renderer::DeathExplosionRendererPlugin, renderer::RendererPlugin};

pub mod renderer;
pub mod death_explosion_renderer;

pub struct MaterialLoaderPlugin;

impl Plugin for MaterialLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RendererPlugin);
        app.add_plugins(DeathExplosionRendererPlugin);
    }
}


