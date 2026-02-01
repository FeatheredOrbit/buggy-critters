use bevy::prelude::*;

pub enum LargestSightUpdateType {
    SpawnInit,
    EntityDied,
    EntityBorn(Entity)
}

#[derive(Event)]
pub struct UpdateLargestSight(pub LargestSightUpdateType);