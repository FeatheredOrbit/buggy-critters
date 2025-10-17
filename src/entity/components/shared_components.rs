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

// Action states
#[derive(Component)]
pub struct Idle;

// Searching states
#[derive(Component)]
pub struct SearchingNew;

// Moving states
#[derive(Component)]
pub struct MovingNew;

