use bevy::prelude::*;

use crate::entity::components::shared_components::*;
use crate::entity::components::utils_components::Velocity;
use crate::materials::entity_materials::*;

pub fn update_entity_material(query: Query<(&Children, &Velocity), With<Moving>>, material_handles: Query<&MeshMaterial2d<FuzzMaterial>>, mut materials: ResMut<Assets<FuzzMaterial>>) {
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