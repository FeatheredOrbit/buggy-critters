use bevy::prelude::*;

pub enum LargestSightUpdateType {
    SpawnInit,
    EntityDied,
    EntityBorn(Entity)
}

#[derive(Event)]
pub struct UpdateLargestSightEvent(pub LargestSightUpdateType);


#[derive(Event)]
pub struct HandleEntityDeathEvent(pub (Entity, Transform)); 