use std::collections::HashMap;

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod bug_entity;
mod scene;
mod food;
mod constants;
mod utils;
mod compute;

mod materials;
mod resources;

use bug_entity::EntityPlugin;
use food::FoodPlugin;
use scene::SceneManagerPlugin;
use materials::MaterialLoaderPlugin;

use resources::{EntityGrid, FruitGrid, CurrentlySelectedEntity};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(EntityGrid(HashMap::new()))
        .insert_resource(FruitGrid(HashMap::new()))
        .insert_resource(CurrentlySelectedEntity(None))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                name: Some("Buggy Critters".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }))
        
        .add_plugins(MaterialLoaderPlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(EntityPlugin)
        .run();
}
