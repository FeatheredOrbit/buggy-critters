use bevy::prelude::*;

use crate::food::fruit_entity::components::*;
use crate::materials::renderer::resources::RendererHandle;

pub fn spawn
(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    renderer_handle: Res<RendererHandle>
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
            MeshMaterial2d(renderer_handle.0.clone()),
            Visibility::Visible
        ));

    }
}