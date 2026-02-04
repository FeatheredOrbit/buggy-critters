use bevy::render::render_resource::ShaderType;
use bytemuck::{Pod, Zeroable};


#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShaderType)]
pub struct DeathExplosionShaderData {
    // Transform of the explosion
    pub transform: [[f32; 4]; 4],

    // Amount of fragments to render, only first field is important
    pub fragment_amount: [u32; 4]
}