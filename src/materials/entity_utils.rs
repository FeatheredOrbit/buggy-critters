use bevy::render::render_resource::ShaderType;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod, ShaderType)]
pub struct EntityShaderData {
    pub transform: [[f32;4];4],

    pub info1: [f32; 4]
}

impl EntityShaderData {
    pub fn create(trans: [[f32; 4]; 4], vel: f32, head_index: u32, body_index: u32, legs_index: u32) -> Self {
        Self {
            transform: trans,

            info1: [vel, head_index as f32, body_index as f32, legs_index as f32]

        }
    }
}