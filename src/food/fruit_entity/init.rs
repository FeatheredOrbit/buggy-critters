use bevy::prelude::*;
use bevy::render::storage::ShaderStorageBuffer;

use crate::food::fruit_entity::components::*;
use crate::materials::renderer::RendererHandle;

pub fn spawn
(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut renderer_handle: Res<RendererHandle>
) 
{
    let mesh_handle = meshes.add(Mesh::from(Rectangle::new(120.0, 120.0)));

    for i in 0..1 {

        commands.spawn(())

        // Identifier
        .insert(FruitEntityRoot)

        // Transform and GlobalTransform
        .insert((
            Transform::from_xyz(0.0, 0.0, -(i as f32)),
            GlobalTransform::default()
        ))

        // Values
        .insert((

            NutritionalValue(50.0),

        ))

        // Add the renderer and mesh
        .insert((
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(renderer_handle.0.clone())
        ));

    }
}