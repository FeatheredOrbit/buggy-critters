use bevy::camera::visibility::NoFrustumCulling;
use bevy::prelude::*;

use crate::constants::AMOUNT_OF_FRUITS;
use crate::food::fruit_entity::components::*;
use crate::materials::renderer::resources::{RendererHandle, RendererMeshHandle};

pub fn spawn
(
    mut commands: Commands,
    renderer_handle: Res<RendererHandle>,
    renderer_mesh_handle: Res<RendererMeshHandle>
) 
{
    for i in 0..AMOUNT_OF_FRUITS {

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
            Mesh2d(renderer_mesh_handle.0.clone()),
            MeshMaterial2d(renderer_handle.0.clone()),
            NoFrustumCulling
        ));

    }
}