use bevy::render::render_resource::ShaderType;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod, ShaderType)]
pub struct ShaderData {
    // Transform matrix, format is the same for anything.
    pub transform: [[f32;4];4],

    // What does this data represent?
    // Values [0, 1] represent [bug, fruit] respectively.
    pub data_type: u32,

    pub _pad: [u32; 3],

    // First packet of information.
    // For entities, slots [0, 1, 2, 3] are [velocity, head_sprite_index, body_sprite_index, legs_sprite_index] respectively.
    // For fruits, slots [0, 1, 2, 3] are [speed, nothing, nothing, nothing] respectively.
    pub info1: [f32; 4],

    // Second packet of information.
    // For entities, slots [0, 1, 2, 3] are [head_color_r, head_color_g, head_color_b, head_color_a] respectively.
    // For fruits, slots [0, 1, 2, 3] are [color_r, color_g, color_b, color_a] respectively.
    pub info2: [f32; 4],

    // Third packet of information.
    // For entities, slots [0, 1, 2, 3] are [body_color_r, body_color_g, body_color_b, body_color_a] respectively.
    pub info3: [f32; 4], 

    // Fourth packet of information.
    // For entities, slots [0, 1, 2, 3] are [legs_color_r, legs_color_g, legs_color_b, legs_color_a] respectively.
    pub info4: [f32; 4]
}

impl ShaderData {
    pub fn create_for_entity(trans: [[f32; 4]; 4], vel: f32, head_index: u32, body_index: u32, legs_index: u32) -> Self {
        Self {
            transform: trans,

            data_type: 0,

            _pad: [0, 0, 0],

            info1: [vel, head_index as f32, body_index as f32, legs_index as f32],
            info2: [1.0, 0.0, 0.0, 1.0],
            info3: [0.0, 1.0, 0.0, 1.0],
            info4: [0.0, 0.0, 1.0, 1.0]

        }
    }
    
    pub fn create_for_fruit(trans: [[f32; 4]; 4], speed: f32) -> Self {
        Self {
            transform: trans,

            data_type: 1,

            _pad: [0, 0, 0],

            info1: [speed, 0.0, 0.0, 0.0],
            info2: [1.0, 0.0, 0.0, 1.0],
            info3: [0.0, 0.0, 0.0, 0.0],
            info4: [0.0, 0.0, 0.0, 0.0]
        }
    }
}