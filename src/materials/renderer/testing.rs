use bevy::{
    asset::{RenderAssetUsages, uuid_handle},
    color::palettes::basic::YELLOW,
    core_pipeline::core_2d::{CORE_2D_DEPTH_FORMAT, Transparent2d},
    math::{FloatOrd, ops},
    mesh::{Indices, MeshVertexAttribute, VertexBufferLayout},
    prelude::*,
    render::{
        Extract, Render, RenderApp, RenderStartup, RenderSystems, extract_resource::ExtractResourcePlugin, mesh::RenderMesh, render_asset::RenderAssets, render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand, SetItemPipeline, ViewSortedRenderPhases
        }, render_resource::{
            BlendState, BufferAddress, BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Face, FragmentState, MultisampleState, PipelineCache, PrimitiveState, PrimitiveTopology, RenderPipelineDescriptor, SpecializedRenderPipeline, SpecializedRenderPipelines, StencilFaceState, StencilState, TextureFormat, VertexFormat, VertexState, VertexStepMode
        }, renderer::RenderDevice, sync_component::SyncComponentPlugin, sync_world::{MainEntity, MainEntityHashMap, RenderEntity}, view::{ExtractedView, RenderVisibleEntities, ViewTarget}
    },
    sprite_render::{
        DrawMesh2d, Material2dBindGroupId, Mesh2dPipeline, Mesh2dPipelineKey, Mesh2dTransforms, MeshFlags, RenderMesh2dInstance, SetMesh2dBindGroup, SetMesh2dViewBindGroup, extract_mesh2d, init_mesh_2d_pipeline
    },
};
use std::f32::consts::PI;

use crate::materials::renderer::{resources::EntitiesToRender, shader_data::ShaderData};

const SHADER_HANDLE: Handle<Shader> = uuid_handle!("697663f3-d6b1-4852-8156-cc2c87b0d614");

type DrawColoredMesh2d = (
    // Set the pipeline
    SetItemPipeline,
    // Set the view uniform as bind group 0
    SetMesh2dViewBindGroup<0>,
    // Set the mesh uniform as bind group 1
    SetMesh2dBindGroup<1>,
    // Draw the mesh
    RendererDrawCommand
);

struct RendererDrawCommand;
impl<P: PhaseItem> RenderCommand<P> for RendererDrawCommand {
    type ItemQuery = ();
    type ViewQuery = ();
    type Param = ();

    fn render<'w>
    (
        item: &P,
        view: bevy::ecs::query::ROQueryItem<'w, '_, Self::ViewQuery>,
        entity: Option<bevy::ecs::query::ROQueryItem<'w, '_, Self::ItemQuery>>,
        param: bevy::ecs::system::SystemParamItem<'w, '_, Self::Param>,
        pass: &mut bevy::render::render_phase::TrackedRenderPass<'w>,
    ) -> bevy::render::render_phase::RenderCommandResult {
        
        bevy::render::render_phase::RenderCommandResult::Success
    }
}

#[derive(Resource)]
struct CustomRendererPipeline {
    mesh2d_pipeline: Mesh2dPipeline,
    shader: Handle<Shader>
}
impl SpecializedRenderPipeline for CustomRendererPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        // Customize how to store the meshes' vertex attributes in the vertex buffer
        // Our meshes only have position and color
        let formats = vec![
            // Position
            VertexFormat::Float32x3,
            // UV
            VertexFormat::Float32x2,
        ];

        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Instance, formats);

        let format = match key.contains(Mesh2dPipelineKey::HDR) {
            true => ViewTarget::TEXTURE_FORMAT_HDR,
            false => TextureFormat::bevy_default(),
        };

        RenderPipelineDescriptor {
            vertex: VertexState {
                // Use our custom shader
                shader: self.shader.clone(),
                // Use our custom vertex buffer
                buffers: vec![vertex_layout],
                ..default()
            },
            fragment: Some(FragmentState {
                // Use our custom shader
                shader: self.shader.clone(),
                targets: vec![Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
                ..default()
            }),
            // Use the two standard uniforms for 2d meshes
            layout: vec![
                // Bind group 0 is the view uniform
                self.mesh2d_pipeline.view_layout.clone(),
                // Bind group 1 is the mesh uniform
                self.mesh2d_pipeline.mesh_layout.clone(),
            ],
            primitive: PrimitiveState {
                cull_mode: Some(Face::Back),
                topology: key.primitive_topology(),
                ..default()
            },
            depth_stencil: Some(DepthStencilState {
                format: CORE_2D_DEPTH_FORMAT,
                depth_write_enabled: false,
                depth_compare: CompareFunction::GreaterEqual,
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
            }),
            multisample: MultisampleState {
                count: key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("colored_mesh2d_pipeline".into()),
            ..default()
        }
    }
}

pub struct RendererPlugin;
impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        // Load our custom shader
        let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
        
        let shader_raw = include_str!("../../../assets/shaders/special_effects/death_explosion.wgsl");

        shaders.insert(SHADER_HANDLE.id(), Shader::from_wgsl(shader_raw, file!())).unwrap();

        app.add_plugins(ExtractResourcePlugin::<EntitiesToRender>::default());

        // Register our custom draw function, and add our render systems
        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .add_render_command::<Transparent2d, DrawColoredMesh2d>()
            .init_resource::<SpecializedRenderPipelines<CustomRendererPipeline>>()
            .add_systems(
                RenderStartup,
                init.after(init_mesh_2d_pipeline),
            )
            .add_systems(
                Render,
                queue.in_set(RenderSystems::QueueMeshes),
            );
    }
}

fn init(
    mut commands: Commands,
    mesh2d_pipeline: Res<Mesh2dPipeline>,
    device: Res<RenderDevice>
) {

    let instance_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("Instance Buffer"),
        size: (size_of::<ShaderData>() * 15000) as BufferAddress,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        mapped_at_creation: false
    });

    commands.insert_resource(CustomRendererPipeline {
        mesh2d_pipeline: mesh2d_pipeline.clone(),
        shader: SHADER_HANDLE
    });
}

fn queue(
    draw_functions: Res<DrawFunctions<Transparent2d>>,
    pipeline: Res<CustomRendererPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<CustomRendererPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    mut phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
    views: Query<(&ExtractedView, &Msaa)>,
) {
    let draw_fn = draw_functions.read().id::<DrawColoredMesh2d>();

    for (view, msaa) in &views {
        let Some(phase) = phases.get_mut(&view.retained_view_entity) else {
            continue;
        };

        // Obligatory pipeline key junk (this is wgpu reality, not Bevy fluff)
        let key =
            Mesh2dPipelineKey::from_msaa_samples(msaa.samples()) |
            Mesh2dPipelineKey::from_hdr(view.hdr);

        let pipeline_id =
            pipelines.specialize(&pipeline_cache, &pipeline, key);

        phase.add(Transparent2d {
            entity: (Entity::PLACEHOLDER, MainEntity::from(Entity::PLACEHOLDER)),
            pipeline: pipeline_id,
            draw_function: draw_fn,
            sort_key: FloatOrd(0.0),
            batch_range: 0..1,
            extra_index: PhaseItemExtraIndex::None,
            extracted_index: usize::MAX,
            indexed: true,
        });
    }
}
