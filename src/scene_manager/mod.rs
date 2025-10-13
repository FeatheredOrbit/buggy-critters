use bevy::prelude::*;

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

fn init(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 4.0;

    commands.spawn(
        camera
    );
}