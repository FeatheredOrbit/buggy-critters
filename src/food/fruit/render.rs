use bevy::{prelude::*, render::storage::ShaderStorageBuffer};

use crate::{food::fruit::components::{Fruit, NutritionalValue}, materials::{food_materials::FruitRenderer, food_utils::FruitShaderData}};

pub fn update_render
(
    query: Query<(&GlobalTransform, &NutritionalValue), With<Fruit>>,
    material_query: Query<&MeshMaterial2d<FruitRenderer>>,
    mut materials: ResMut<Assets<FruitRenderer>>,
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>,

    time: Res<Time>
)
{
    let mut data_buffer: Vec<FruitShaderData> = vec![];

    for (transform, nutritional_value) in &query {
        data_buffer.push(
            FruitShaderData::create
            (
            transform.compute_transform().to_matrix().to_cols_array_2d(), 
            (1.0 / nutritional_value.0) * 250.0
            )
        );
    }

    if data_buffer.len() > 0 {

        if let Some(mat_handle) = material_query.iter().next() {
            if let Some(mat) = materials.get_mut(mat_handle) {

                mat.time = time.elapsed_secs();

                if let Some(storage_buffer) = storage_buffers.get_mut(&mat.fruits) {
                    storage_buffer.set_data(data_buffer);
                }

            }
        }

    }
}