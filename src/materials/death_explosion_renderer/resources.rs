use bevy::prelude::*;

use crate::materials::death_explosion_renderer::DeathExplosionRenderer;

#[derive(Resource)]
pub struct DeathExplosionRendererHandle(pub Handle<DeathExplosionRenderer>);

#[derive(Resource)]
pub struct DeathExplosionMeshHandle(pub Handle<Mesh>);