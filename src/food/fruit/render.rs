use bevy::prelude::*;

use crate::{food::fruit::components::{Fruit, NutritionalValue}, materials::food_materials::StaticMaterial};

pub fn update_entity_material(query: Query<(Entity, &NutritionalValue), With<Fruit>>, material_handles: Query<&MeshMaterial2d<StaticMaterial>>, mut materials: ResMut<Assets<StaticMaterial>>, time: Res<Time>) {
    for (entity, nutritional_value) in &query {
        
        if let Ok(handle) = material_handles.get(entity) {

            if let Some(mat) = materials.get_mut(handle) {

                mat.speed =  (1.0 / nutritional_value.0) * 75.0;
                mat.time = time.elapsed_secs();

            }

        }
    }
}