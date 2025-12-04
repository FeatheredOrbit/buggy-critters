use bevy::prelude::Component;

#[derive(Component)]
pub struct CurrentlyMoving(pub bool);

#[derive(Component)]
pub struct CurrentlyRotating(pub bool);

pub enum MovementPatterns {
    Smooth,
    Continuous,
    ZigZag
}

#[derive(Component)]
pub struct MovementPattern(pub MovementPatterns);