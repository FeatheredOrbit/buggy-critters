use bevy::prelude::*;

pub fn init(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),

        Camera2d,

        Projection::from(OrthographicProjection {
            scale: 2.0,

            ..OrthographicProjection::default_2d()
        })
    ));
}