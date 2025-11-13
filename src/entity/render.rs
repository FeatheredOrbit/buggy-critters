use bevy::prelude::*;
use bevy::render::storage::ShaderStorageBuffer;

use crate::entity::components::render_components::{BodyPartsIndexes, EntityRoot};
use crate::entity::components::shared_components::*;
use crate::entity::components::utils_components::Velocity;
use crate::materials::entity_materials::*;
use crate::materials::entity_utils::EntityShaderData;

pub fn update_entity_material(query: Query<(&Children, &Velocity), With<Moving>>, material_handles: Query<&MeshMaterial2d<EntityMaterial>>, mut materials: ResMut<Assets<EntityMaterial>>) {
    for (children, velocity) in &query {

        for child in children {
            if let Ok(handle) = material_handles.get(*child) {
                if let Some(mat) = materials.get_mut(handle) {
                    mat.velocity = velocity.0.length();
                }
            }
        }
    }
}

pub fn sigmaboi
(
    query: Query<(&GlobalTransform, &Velocity, &BodyPartsIndexes), With<EntityRoot>>, 
    material_query: Query<&MeshMaterial2d<SigmaBludMaterial>>,
    mut materials: ResMut<Assets<SigmaBludMaterial>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>
) 
{
    let mut buffer: Vec<EntityShaderData> = vec![];

    for (transform, velocity, indexes) in &query {
        let data = EntityShaderData::create( 
            transform.compute_transform().to_matrix().to_cols_array_2d(), 
            velocity.0.to_array(),
            indexes.head,
            indexes.body,
            indexes.legs
        );

        buffer.push(data);
    }

    if let Some(mat_handle) = material_query.iter().next() {
        if buffer.len() > 0 {
            if let Some(mat) = materials.get_mut(mat_handle) {

                if let Some(buffer_asset) = storage_buffers.get_mut(&mat.entities) {
                    buffer_asset.set_data(buffer.clone());
                }

            }
        }
    }

    
}