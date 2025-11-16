use bevy::prelude::*;
use rand::{thread_rng, Rng};

////////////////////////////////////////////////////////////////////////////////////////
// Utils
////////////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct FutureTransform {
    pub position: Vec3,
    pub angle: Quat
}

////////////////////////////////////////////////////////////////////////////////////////
// Entity variables
////////////////////////////////////////////////////////////////////////////////////////
pub enum MovementPatterns {
    Smooth,
    Continuous,
    ZigZag
}

#[derive(Component)]
pub struct MovementPattern(pub MovementPatterns);

#[derive(Component)]
pub struct PhysicalTraits {
    pub size: f32,
    pub sight: f32,
    pub speed: f32,
    pub reach: f32
}

impl PhysicalTraits {
    pub fn new() -> Self {
        let mut rng = thread_rng();

        let size = 1.0;
        
        let sight = rng.gen_range(500.0..700.0) * size;
        let speed = rng.gen_range(1.0..3.0) * size;

        let reach = rng.gen_range(0.5..2.0) * size;

        Self {
            size,
            sight,
            speed,
            reach
        }
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


