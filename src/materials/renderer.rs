use bevy::{
    asset::uuid_handle, prelude::*, render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer}, shader::ShaderRef, sprite_render::{AlphaMode2d, Material2d}
};

pub const SHADER_HANDLE: Handle<Shader> = uuid_handle!("38c96d71-9b05-467e-b646-3380f0bdf860");

#[derive(Resource)]
pub struct RendererHandle(pub Handle<Renderer>);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct Renderer {
    #[storage(0, read_only)]
    pub entities: Handle<ShaderStorageBuffer>,

    #[texture(1)]
    #[sampler(2)]
    pub atlas_texture: Handle<Image>,

    #[texture(3)]
    #[sampler(4)]
    pub noise_texture: Handle<Image>
}

impl Material2d for Renderer {
    fn fragment_shader() -> ShaderRef {
        return ShaderRef::Handle(SHADER_HANDLE);
    }

    fn vertex_shader() -> ShaderRef {
        return ShaderRef::Handle(SHADER_HANDLE);
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}
