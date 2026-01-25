use bevy::prelude::Component;

#[derive(Component)]
pub struct BugEntityRoot;

#[derive(Component)]
pub struct BodyPartsIndexes {
    pub head: u32,
    pub body: u32,
    pub legs: u32
}