use bevy::prelude::*;
use bevy::render::storage::ShaderStorageBuffer;

use crate::entity::components::render_components::{BodyPartsIndexes, EntityRoot};
use crate::entity::components::utils_components::Velocity;
use crate::materials::entity_materials::*;
use crate::materials::entity_utils::EntityShaderData;

pub fn update_render
(
    query: Query<(&GlobalTransform, &Velocity, &BodyPartsIndexes), With<EntityRoot>>, 
    material_query: Query<&MeshMaterial2d<EntityRenderer>>,
    mut materials: ResMut<Assets<EntityRenderer>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>
) 
{
    let mut buffer: Vec<EntityShaderData> = vec![];

    for (transform, velocity, indexes) in &query {
        let data = EntityShaderData::create( 
            transform.compute_transform().to_matrix().to_cols_array_2d(), 
            velocity.0.length(),
            indexes.head,
            indexes.body,
            indexes.legs
        );

        buffer.push(data);
    }

    if buffer.len() > 0 {

        if let Some(mat_handle) = material_query.iter().next() {
        
            if let Some(mat) = materials.get_mut(mat_handle) {

                if let Some(buffer_asset) = storage_buffers.get_mut(&mat.entities) {
                    buffer_asset.set_data(buffer.clone());
                }

            }
        }

    }

    
}