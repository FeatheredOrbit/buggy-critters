use bevy::prelude::Component;

#[derive(Component)]
pub struct CurrentlyMoving(pub bool);

#[derive(Component)]
pub struct CurrentlyRotating(pub bool);