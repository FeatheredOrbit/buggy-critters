use bevy::prelude::*;

use crate::materials::entity_materials::*;

pub fn update_entity_material(mut materials: ResMut<Assets<FuzzMaterial>>, time: Res<Time>) {
    for (_, material) in materials.iter_mut() {
        material.time = time.elapsed_secs();
    }
}