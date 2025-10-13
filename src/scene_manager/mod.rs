use bevy::prelude::*;

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

fn init(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle::default()
    );
}