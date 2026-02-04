use bevy::{asset::RenderAssetUsages, mesh::MeshVertexAttribute, prelude::*, render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer}, sprite_render::{Material2d, Material2dPlugin}};

use crate::{constants::ENTITY_DEFAULT_SIZE, materials::death_explosion_renderer::{render::update_render, resources::{DeathExplosionMeshHandle, DeathExplosionRendererHandle}, shader_data::DeathExplosionShaderData}};

pub mod resources;
pub mod components;
mod render;
mod update;
mod shader_data;

const FRAGMENT_ID: MeshVertexAttribute = MeshVertexAttribute::new("Fragment_Ids", 3, bevy::mesh::VertexFormat::Float32);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct DeathExplosionRenderer {
    #[storage(0, read_only)]
    pub dead_entities: Handle<ShaderStorageBuffer>
}
impl Material2d for DeathExplosionRenderer {
    fn vertex_shader() -> bevy::shader::ShaderRef {
        return "shaders/special_effects/death_explosion.wgsl".into();
    }
    fn fragment_shader() -> bevy::shader::ShaderRef {
        return "shaders/special_effects/death_explosion.wgsl".into();
    }
    fn alpha_mode(&self) -> bevy::sprite_render::AlphaMode2d {
        return bevy::sprite_render::AlphaMode2d::Blend;
    }
    fn specialize(
            descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
            layout: &bevy::mesh::MeshVertexBufferLayoutRef,
            _key: bevy::sprite_render::Material2dKey<Self>,
        ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {

        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            FRAGMENT_ID.at_shader_location(3)
        ])?;

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

pub struct DeathExplosionRendererPlugin;
impl Plugin for DeathExplosionRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DeathExplosionRenderer>::default());

        app.add_systems(Startup, compile_and_init);
        app.add_systems(Update, update_render);
    }
}

fn compile_and_init
(
    mut commands: Commands, 
    mut render: ResMut<Assets<DeathExplosionRenderer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>
) 
{
    let data= vec![
        DeathExplosionShaderData {
            transform: [[0.0; 4]; 4],
            fragment_amount: [0; 4]
        }
    ];

    let buffer = storage_buffers.add(ShaderStorageBuffer::from(data));

    let death_explosion_renderer_handle = render.add(DeathExplosionRenderer {
            dead_entities: buffer.clone()
        }
    );

    let mesh_handle = meshes.add(create_split_quad_mesh(8));

    let materials = commands.spawn((
        Mesh2d(mesh_handle.clone()),

        MeshMaterial2d(death_explosion_renderer_handle.clone())

    )).id();

    commands.insert_resource(DeathExplosionRendererHandle(death_explosion_renderer_handle));
    commands.insert_resource(DeathExplosionMeshHandle(mesh_handle));

    commands.entity(materials).despawn();
}

// Yeah this was generated with AI, I have to be real but I couldn't be bothered aetting them up manually I just want them immediately for the shader.
pub fn create_split_quad_mesh(splits: usize) -> Mesh {
    let vertex_count = splits + 1;
    let step = 2.0 / splits as f32; // local space step
    let uv_step = 1.0 / splits as f32;

    let mut positions = Vec::new();
    let mut uvs = Vec::new();
    let mut fragment_ids = Vec::new();

    // Generate vertices
    for row in 0..vertex_count {
        let y = 1.0 - row as f32 * step; // top to bottom
        let uv_y = row as f32 * uv_step;

        for col in 0..vertex_count {
            let x = -1.0 + col as f32 * step; // left to right
            let uv_x = col as f32 * uv_step;

            positions.push([x, y, 0.0]);
            uvs.push([uv_x, uv_y]);

            // fragment_id = which small quad this vertex belongs to (top-left corner of quads)
            // Each quad has row,col = 0..(splits-1)
            // To assign per-vertex, we take the top-left quad of the vertex
            let frag_row = if row == splits { row - 1 } else { row };
            let frag_col = if col == splits { col - 1 } else { col };
            let fragment_id = frag_row * splits + frag_col;
            fragment_ids.push(fragment_id as f32); // as float to send to shader
        }
    }

    // Generate indices for all quads
    let mut indices = Vec::new();
    for row in 0..splits {
        for col in 0..splits {
            let top_left = row * vertex_count + col;
            let top_right = top_left + 1;
            let bottom_left = top_left + vertex_count;
            let bottom_right = bottom_left + 1;

            // Triangle 1
            indices.push(top_left as u32);
            indices.push(bottom_left as u32);
            indices.push(top_right as u32);

            // Triangle 2
            indices.push(top_right as u32);
            indices.push(bottom_left as u32);
            indices.push(bottom_right as u32);
        }
    }

    let mut mesh = Mesh::new(bevy::mesh::PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    mesh.scale_by(Vec3::new(ENTITY_DEFAULT_SIZE.0, ENTITY_DEFAULT_SIZE.1, 1.0));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh.insert_attribute(FRAGMENT_ID, fragment_ids);

    mesh.insert_indices(bevy::mesh::Indices::U32(indices));

    return mesh;
}