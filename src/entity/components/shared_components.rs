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

#[derive(PartialEq, Eq)]
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

#[derive(Component)]
pub struct NextState(pub States);

// Action states
#[derive(Component)]
pub struct Action;

#[derive(Component)]
pub struct Idle;

// Searching states
#[derive(Component)]
pub struct Searching;

#[derive(Component)]
pub struct SearchingNew;

#[derive(Component)]
pub struct SearchingFood;

// Moving states
#[derive(Component)]
pub struct Moving;

#[derive(Component)]
pub struct MovingNew;

#[derive(Component)]
pub struct MovingFood;


