use std::collections::HashMap;

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod bug_entity;
mod scene;
mod food;
mod constants;
mod utils;
mod events;

mod materials;
mod resources;

use bevy::window::WindowResolution;
use bug_entity::EntityPlugin;
use food::FoodPlugin;
use scene::SceneManagerPlugin;
use materials::MaterialLoaderPlugin;

use resources::{EntityGrid, FruitGrid, CurrentlySelectedEntity};

use crate::events::SetupEventsPlugin;
use crate::resources::LargestEntitySight;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LargestEntitySight::default())
        .insert_resource(EntityGrid(HashMap::new()))
        .insert_resource(FruitGrid(HashMap::new()))
        .insert_resource(CurrentlySelectedEntity(None))

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Buggy Critters".to_string(),
                resizable: false,
                resolution: WindowResolution::new(1200, 800),
                ..Default::default()
            }),
            ..Default::default()
        }))
        
        .add_plugins((
            FrameTimeDiagnosticsPlugin::default(), 
            LogDiagnosticsPlugin::default()
        ))
        .add_plugins(SetupEventsPlugin)
        .add_plugins(MaterialLoaderPlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(EntityPlugin)
        .run();
}
