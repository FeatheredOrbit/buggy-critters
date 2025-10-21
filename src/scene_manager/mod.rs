use bevy::prelude::*;

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

fn init(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),

        Camera2d,

        Projection::from(OrthographicProjection {
            scale: 1.0,

            ..OrthographicProjection::default_2d()
        })
    ));
}