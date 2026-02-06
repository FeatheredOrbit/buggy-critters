use std::collections::HashMap;

use bevy::{prelude::*, render::{extract_resource::ExtractResource, render_resource::{BindGroup, Buffer}}};

use crate::materials::renderer::{Renderer, shader_data::ShaderData};

#[derive(Resource)]
pub struct RendererHandle(pub Handle<Renderer>);

#[derive(Resource)]
pub struct RendererMeshHandle(pub Handle<Mesh>);

#[derive(Resource, Default, ExtractResource, Clone)]
pub struct EntitiesToRender {
    pub data: HashMap<Entity, ShaderData>,
    pub dirty: bool
}

#[derive(Resource)]
pub struct InstanceBuffer {
    pub buffer: Buffer,
    pub instance_count: usize
}