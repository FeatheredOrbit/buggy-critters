use bevy::{asset::RenderAssetUsages, mesh::{MeshVertexAttribute, PrimitiveTopology}, prelude::*, render::{render_resource::AsBindGroup, storage::ShaderStorageBuffer}, sprite_render::{Material2d, Material2dPlugin}};

use crate::materials::death_explosion_renderer::resources::{DeathExplosionMeshHandle, DeathExplosionRendererHandle};

pub mod resources;

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
}

pub struct DeathExplosionRendererPlugin;
impl Plugin for DeathExplosionRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<DeathExplosionRenderer>::default());

        app.add_systems(Startup, compile_and_init);
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
    let data: Vec<f32> = vec![];

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

// Yeah this was generated with AI, I have NO ABSOLUTE IDEA how to properly set up positions, uvs and other stuff, surprised I can even work with shaders in general.
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

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(MeshVertexAttribute::new("Fragment IDs", 3, bevy::mesh::VertexFormat::Float32) , fragment_ids);

    mesh.insert_indices(bevy::mesh::Indices::U32(indices));

    return mesh;
}