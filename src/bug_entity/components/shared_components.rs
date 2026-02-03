use bevy::prelude::*;
use rand::{SeedableRng, rngs::SmallRng};

////////////////////////////////////////////////////////////////////////////////////////
// Utils
////////////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct FutureTransform {
    pub position: Vec3,
    pub angle: Quat
}

#[derive(Component)]
pub struct RngComponent(pub SmallRng);
impl Default for RngComponent {
    fn default() -> Self {
        Self(SmallRng::from_os_rng())
    }
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

// Signals requirement for a change in state and stores the next state.
#[derive(Component)]
pub struct StateChangeRequired(pub States);

// Stores the current state.
#[derive(Component)]
pub struct CurrentState(pub States);


// Action states

// Common identifier
#[derive(Component)]
pub struct Action;

//Specific identifiers
#[derive(Component)]
pub struct Idling;


// Searching states

// Common identifier
#[derive(Component)]
pub struct Searching;

//Specific identifiers
#[derive(Component)]
pub struct SearchingNew;

#[derive(Component)]
pub struct SearchingFood;


// Moving states

// Common identifier
#[derive(Component)]
pub struct Moving;

//Specific identifiers
#[derive(Component)]
pub struct MovingNew;

#[derive(Component)]
pub struct MovingFood;


// Other states

#[derive(Component)]
pub struct Dead {
    pub time_since_animation: f32,
    pub animation_acceleration: f32
}
impl Default for Dead {
    fn default() -> Self {
        Self {
            time_since_animation: 0.0,
            animation_acceleration: 1.0 / 35.0
        }
    }
}