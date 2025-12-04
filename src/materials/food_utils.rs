use bevy::render::render_resource::ShaderType;

#[repr(C)]
#[derive(bytemuck::Zeroable, bytemuck::Pod, ShaderType, Debug, Clone, Copy)]
pub struct FruitShaderData {
    transform: [[f32; 4]; 4],

    info1: [f32; 4]
}

impl FruitShaderData {
    pub fn create(trans: [[f32; 4]; 4], speed: f32) -> Self {
        Self {
            transform: trans,
            info1: [speed, 0.0, 0.0, 0.0]
        }
    }
}