use bevy::camera::visibility::NoFrustumCulling;
use bevy::prelude::*;

use crate::constants::{AMOUNT_OF_FRUITS, FRUIT_DEFAULT_SIZE};
use crate::food::fruit_entity::components::*;
use crate::materials::renderer::resources::RendererHandle;

pub fn spawn
(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    renderer_handle: Res<RendererHandle>
) 
{
    let mesh_handle = meshes.add(Mesh::from(Rectangle::new(FRUIT_DEFAULT_SIZE.0, FRUIT_DEFAULT_SIZE.1)));

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
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(renderer_handle.0.clone()),
            NoFrustumCulling
        ));

    }
}