use bevy::prelude::*;
use rand::Rng;

use crate::resources::GlobalRng;

#[derive(Component)]
pub struct DeathExplosion {
    pub duration: f32,
    pub fragment_amount: u32
}
impl DeathExplosion {
    pub fn new(mut rng: ResMut<GlobalRng>) -> Self {
        let amount = rng.0.random_range(3..=8) as u32;

        Self {
            duration: 1.0,
            fragment_amount: amount
        }
    }
}