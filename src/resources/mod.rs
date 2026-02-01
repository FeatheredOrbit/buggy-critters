use bevy::prelude::*;
use std::collections::HashMap;


#[derive(Resource)]
pub struct EntityGrid(pub HashMap<(i32, i32), Vec<Entity>>);

#[derive(Resource)]
pub struct FruitGrid(pub HashMap<(i32, i32), Vec<Entity>>);

#[derive(Resource)]
pub struct CurrentlySelectedEntity(pub Option<Entity>);