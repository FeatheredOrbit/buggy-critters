use bevy::{prelude::*, render::storage::ShaderStorageBuffer};
use crate::{constants::{CHUNKY_BODY_ATLAS_INDEX, CHUNKY_HEAD_ATLAS_INDEX, CURVED_LEGS_ATLAS_INDEX}, entity::components::{debug_components::*, idle_components::*, moving_components::*, render_components::*, shared_components::*, utils_components::*}, materials::entity_utils::EntityShaderData};

use crate::materials::entity_materials::*;


pub fn spawn
(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut storage_buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<EntityRenderer>>
) 
{

    let data: Vec<EntityShaderData> = vec![
        
    ];

    let buffer = storage_buffers.add(ShaderStorageBuffer::from(data));

    let atlas: Handle<Image> = asset_server.load("art/bugs/body_parts/atlas.png");
    let noise: Handle<Image> = asset_server.load("art/other/noise_texture.png");

    let renderer_handle = mat.add(EntityRenderer {
            entities: buffer,
            atlas_texture: atlas,
            noise_texture: noise
        }
    );

    let mesh = Mesh::from(Rectangle::new(120.0, 120.0));

    let mesh_handle = meshes.add(mesh);
        
    for i in 0..5 {
        commands.spawn(())
        
        // Its transform component
        .insert((
            Transform::from_xyz(0.0, 0.0, -i as f32),
            GlobalTransform::default(),
            InheritedVisibility::default()
        ))

        // Identifier for the parent
        .insert(EntityRoot)

        // Physical traits
        .insert(PhysicalTraits::new())
    
        // Component for handling what state to move to and the current state
        .insert((
            NextState(crate::entity::components::shared_components::States::None),
            CurrentState(crate::entity::components::shared_components::States::Idle)
        ))

        // Starting state
        .insert((
        Action,
        Idle
    ))

        // Initialize components for idle state
        .insert((
        TimeToAction::new(),
        ActionTimer::new(),

        IdleBehaviours(vec![
            IdleBehaviour{name: IdleStates::Move, weight: 3},
            IdleBehaviour{name: IdleStates::Stay, weight: 7},
            IdleBehaviour{name: IdleStates::SearchFood, weight: 2}
        ])
    ))

        // Initialize components for moving states
        .insert((
        CurrentlyRotating(true),
        CurrentlyMoving(false)
    ))

        // Initialize components for searching states and moving states
        .insert((
        MovementPattern(MovementPatterns::Smooth),
        FutureTransform{position: Vec3::default(), angle: Quat::default()}
    ))

        // Components that holds the index of the body parts on the sprite atlas
        .insert ( BodyPartsIndexes {
            head: CHUNKY_HEAD_ATLAS_INDEX,
            body: CHUNKY_BODY_ATLAS_INDEX,
            legs: CURVED_LEGS_ATLAS_INDEX
        } )

        // Debug components
        .insert(DrawSightRadius)

        // Utils components
        .insert((
            PreviousTransform(Vec2 { x: (0.0), y: (0.0) }),
            Velocity(Vec2 { x: (0.0), y: (0.0) })
        ))

        // Rendering logic
        .insert((
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(renderer_handle.clone())
        ));

    }
}
