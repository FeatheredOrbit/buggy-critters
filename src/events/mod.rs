use bevy::{camera::visibility::NoFrustumCulling, ecs::relationship::RelationshipSourceCollection, prelude::*};

use crate::{bug_entity::components::{attribute_components::PhysicalTraits, render_components::BugEntityRoot}, events::labels::{HandleEntityDeathEvent, LargestSightUpdateType, UpdateLargestSightEvent}, materials::{death_explosion_renderer::{components::DeathExplosion, resources::{DeathExplosionMeshHandle, DeathExplosionRendererHandle}}, renderer::resources::EntitiesToRender}, resources::{CurrentlySelectedEntity, GlobalRng, LargestEntitySight}};

pub mod labels;

pub struct SetupEventsPlugin;
impl Plugin for SetupEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(update_largest_sight_event);
        app.add_observer(handle_entity_death_event);
    }
}

pub fn update_largest_sight_event(
    event: On<UpdateLargestSightEvent>,
    query: Query<(Entity, &PhysicalTraits), With<BugEntityRoot>>,
    mut largest_entity_sight: ResMut<LargestEntitySight>
) {
    match event.0 {
        LargestSightUpdateType::SpawnInit => {
            let mut new_largest_sight = 0.0;
            let mut new_entity = Entity::new();

            for (entity, traits) in &query {
                if traits.sight > new_largest_sight {
                    new_largest_sight = traits.sight;
                    new_entity = entity;
                }
            }

            largest_entity_sight.radius = Some(new_largest_sight); 
            largest_entity_sight.entity = Some(new_entity);
        },

        LargestSightUpdateType::EntityDied => {
            let mut new_largest_sight = 0.0;
            let mut new_entity = Entity::new();

            for (entity, traits) in &query {
                if traits.sight > new_largest_sight {
                    new_largest_sight = traits.sight;
                    new_entity = entity;
                }
            }

            largest_entity_sight.radius = Some(new_largest_sight); 
            largest_entity_sight.entity = Some(new_entity);
        },


        LargestSightUpdateType::EntityBorn(entity) => {
            if let (Ok((entity, traits)), Some(largest_sight_radius)) = (query.get(entity), largest_entity_sight.radius.as_mut()) {
                if traits.sight > *largest_sight_radius {
                    *largest_sight_radius = traits.sight;
                    largest_entity_sight.entity = Some(entity);
                }
            }
        }
    }
}

pub fn handle_entity_death_event(
    event: On<HandleEntityDeathEvent>,
    mut entities_to_render: ResMut<EntitiesToRender>,
    largest_entity_sight: Res<LargestEntitySight>,
    mut selected_entity: ResMut<CurrentlySelectedEntity>,
    global_rng: ResMut<GlobalRng>,
    explosion_renderer: Res<DeathExplosionRendererHandle>,
    explosion_mesh: Res<DeathExplosionMeshHandle>,
    mut commands: Commands
) {
    let (entity, transform) = event.0;

    if let Some(_) = entities_to_render.data.remove(&entity) {}

    if selected_entity.0 == Some(entity) {
        selected_entity.0 = None;
    }

    if let Some(ent) = largest_entity_sight.entity {
        if ent == entity {
            commands.trigger(UpdateLargestSightEvent(LargestSightUpdateType::EntityDied));
        }
    }

    commands.spawn((
        DeathExplosion::new(global_rng),
        GlobalTransform::default(),
        transform,
        NoFrustumCulling,

        Mesh2d(explosion_mesh.0.clone()),
        MeshMaterial2d(explosion_renderer.0.clone())
    ));

    commands.entity(entity).despawn();
}