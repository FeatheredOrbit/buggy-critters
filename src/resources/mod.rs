use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct LargestEntitySight {
    pub entity: Option<Entity>,
    pub radius: Option<f32>
}

#[derive(Resource)]
pub struct EntityGrid(pub HashMap<(i32, i32), Vec<(Entity, Transform)>>);

#[derive(Resource)]
pub struct FruitGrid(pub HashMap<(i32, i32), Vec<(Entity, Transform)>>);

#[derive(Resource)]
pub struct CurrentlySelectedEntity(pub Option<Entity>);