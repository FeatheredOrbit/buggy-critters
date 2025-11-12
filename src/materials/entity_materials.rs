use bevy::{
    prelude::*, render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer}, shader::ShaderRef, sprite_render::{AlphaMode2d, Material2d}
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct EntityMaterial {
    #[uniform(0)]
    pub material_color: LinearRgba,

    #[uniform(1)]
    pub velocity: f32,

    #[texture(2)]
    #[sampler(3)]
    pub main_tex: Handle<Image>,

    #[texture(4)]
    #[sampler(5)]
    pub noise_tex: Handle<Image>
}

impl Material2d for EntityMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shaders/entity.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SigmaBludMaterial {
    #[storage(0, read_only)]
    pub entities: Handle<ShaderStorageBuffer>,

    #[texture(1)]
    #[sampler(2)]
    pub atlas_texture: Handle<Image>
}

impl Material2d for SigmaBludMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shaders/trial.wgsl".into();
    }

    fn vertex_shader() -> ShaderRef {
        return "shaders/trial.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }

}