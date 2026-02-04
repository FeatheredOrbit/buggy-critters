use bevy::prelude::*;
use rand::{SeedableRng, rngs::SmallRng};
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

#[derive(Resource)]
pub struct GlobalRng(pub SmallRng);
impl Default for GlobalRng {
    fn default() -> Self {
        Self(SmallRng::from_os_rng())
    }
}

pub struct GlobalResourcesPlugin;
impl Plugin for GlobalResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LargestEntitySight::default());
        app.insert_resource(EntityGrid(HashMap::new()));
        app.insert_resource(FruitGrid(HashMap::new()));
        app.insert_resource(CurrentlySelectedEntity(None));
        app.insert_resource(GlobalRng::default());
    }
}