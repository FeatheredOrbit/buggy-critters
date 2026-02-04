use std::collections::HashMap;

use bevy::prelude::*;

use crate::materials::renderer::{Renderer, shader_data::ShaderData};

#[derive(Resource)]
pub struct RendererHandle(pub Handle<Renderer>);

#[derive(Resource)]
pub struct RendererMeshHandle(pub Handle<Mesh>);

#[derive(Resource, Default)]
pub struct EntitiesToRender {
    pub data: HashMap<Entity, ShaderData>,
    pub dirty: bool
}