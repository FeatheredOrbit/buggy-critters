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

use crate::events::SetupEventsPlugin;
use crate::resources::GlobalResourcesPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))

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
        .add_plugins(GlobalResourcesPlugin)
        .add_plugins(SetupEventsPlugin)
        .add_plugins(MaterialLoaderPlugin)
        .add_plugins(SceneManagerPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(EntityPlugin)
        .run();
}
