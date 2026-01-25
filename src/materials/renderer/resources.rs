use std::collections::HashMap;

use bevy::prelude::*;

use crate::materials::renderer::{Renderer, shader_data::ShaderData};

#[derive(Resource)]
pub struct RendererHandle(pub Handle<Renderer>);

#[derive(Resource, Default)]
pub struct EntitiesToRender {
    pub data: Vec<ShaderData>,
    pub indexes: HashMap<Entity, usize>
}