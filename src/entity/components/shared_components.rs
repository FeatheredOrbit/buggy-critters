use bevy::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////
// Utils
////////////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct FutureTransform {
    pub position: Vec3,
    pub angle: Quat
}

////////////////////////////////////////////////////////////////////////////////////////
// States Structs
////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Debug)]
pub enum States {
    Idle,
    SearchingNew,
    SearchingFood,
    MovingNew,
    MovingFood,
    None
}

#[derive(Component)]
pub struct CurrentState(pub States);

#[derive(Component, Debug)]
pub struct NextState(pub States);

// Searching states

// Common identifier
#[derive(Component)]
pub struct Searching;

#[derive(Component)]
pub struct SearchingNewBundle;

#[derive(Component)]
pub struct SearchingFoodBundle;

// Moving states

// Common identifier
#[derive(Component)]
pub struct Moving;

#[derive(Component)]
pub struct MovingNewBundle;

#[derive(Component)]
pub struct MovingFoodBundle;


