use bevy::prelude::Component;

#[derive(Component)]
pub struct EntityRoot;

#[derive(Component)]
pub struct EntityRenderer;

#[derive(Component)]
pub struct BodyPartsIndexes {
    pub head: u32,
    pub body: u32,
    pub legs: u32
}