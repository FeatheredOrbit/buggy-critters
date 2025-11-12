use bevy::prelude::*;

use crate::scene::components::MainCamera;

pub fn init(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),

        Camera2d,
        Camera::default(),

        MainCamera,

        Projection::from(OrthographicProjection {
            scale: 1.0,

            ..OrthographicProjection::default_2d()
        })
    ));
}