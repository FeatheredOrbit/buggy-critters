use bevy::{render::render_resource::ShaderType, transform::components::GlobalTransform};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod, ShaderType)]
pub struct EntityShaderData {
    pub transform: [[f32;4];4],

    pub head_atlas_index: u32,
    pub body_atlas_index: u32,
    pub legs_atlas_index: u32,

    _pad: u32
}

impl EntityShaderData {
    pub fn default() -> Self {
        Self {
            transform: GlobalTransform::default().compute_transform().to_matrix().to_cols_array_2d(), 

            head_atlas_index: 1,
            body_atlas_index: 2,
            legs_atlas_index: 3,
            _pad: 1
        }
    }

    pub fn create(trans: [[f32; 4]; 4], head_index: u32, body_index: u32, legs_index: u32) -> Self {
        Self {
            transform: trans,
            head_atlas_index: head_index,
            body_atlas_index: body_index,
            legs_atlas_index: legs_index,
            _pad: 1

        }
    }
}