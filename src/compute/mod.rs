use bevy::{
    prelude::*,
    render::{
        Render, RenderApp, RenderStartup, RenderSystems, extract_resource::{ExtractResource, ExtractResourcePlugin}, graph::CameraDriverLabel, render_graph::{self, NodeRunError, RenderGraph, RenderLabel}, render_resource::*, renderer::{RenderContext, RenderDevice, RenderQueue}
    }
};
use std::borrow::Cow;


const DISPLAY_FACTOR: u32 = 4;
const SIZE: UVec2 = UVec2::new(1280 / DISPLAY_FACTOR, 720 / DISPLAY_FACTOR);
const WORKGROUP_SIZE: u32 = 8;

#[derive(Resource, Clone, ExtractResource, ShaderType)]
struct EntityData {
    data: [i32; 20]
}

#[derive(Resource)]
struct BindGroups([BindGroup; 1]);

#[derive(Resource)]
struct ComputePipeline {
    bind_group_layout: BindGroupLayoutDescriptor,
    pipeline: CachedComputePipelineId
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct NodeLabel;

pub struct ComputePlugin;
impl Plugin for ComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<EntityData>::default());

        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_systems(RenderStartup, init_pipeline);
        render_app.add_systems(Render, prepare_bind_group.in_set(RenderSystems::PrepareBindGroups));

        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(NodeLabel, Node::default());
        render_graph.add_node_edge(NodeLabel, CameraDriverLabel);

    }
}

fn init_pipeline(mut commands: Commands, asset_server: Res<AssetServer>, pipeline_cache: Res<PipelineCache>) {
    let bind_group_layout = BindGroupLayoutDescriptor::new(
        "Compute Group Layout 0",
        &[
            BindGroupLayoutEntry {
                visibility: ShaderStages::COMPUTE,
                binding: 0,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None
            }
        ]
    );

    let shader: Handle<Shader> = asset_server.load("shaders/compute/main.wgsl");

    let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some(Cow::from("Compute Pipeline")),
        layout: vec![bind_group_layout.clone()],
        shader: shader.clone(),
        entry_point: Some(Cow::from("main")),
        ..Default::default()
    });

    commands.insert_resource(ComputePipeline {
        bind_group_layout,
        pipeline
    });
}

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<ComputePipeline>,
    entity_data: Res<EntityData>,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
    queue: Res<RenderQueue>
) {
    let mut storage_buffer = StorageBuffer::from(entity_data.into_inner());
    storage_buffer.write_buffer(&render_device, &queue);

    let bind_group = render_device.create_bind_group(
        None, 
        &pipeline_cache.get_bind_group_layout(&pipeline.bind_group_layout), 
        &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(BufferBinding {buffer: storage_buffer.buffer().unwrap(), offset: 0, size: None})
        }]
    );

    commands.insert_resource(BindGroups([bind_group]));
}

enum State {
    Loading,
    Init,
    Update(usize),
}

struct Node {
    state: State,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            state: State::Loading,
        }
    }
}

impl render_graph::Node for Node {
    fn run(&self, _graph: &mut render_graph::RenderGraphContext, render_context: &mut RenderContext, world: &World) -> Result<(), NodeRunError> {

        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ComputePipeline>();
        let bind_groups = world.resource::<BindGroups>();

        // If the pipeline isn't ready yet, do nothing this frame
        let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipeline.pipeline) else {
            return Ok(());
        };

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_groups.0[0], &[]);

        // One workgroup is enough for 20 integers
        pass.dispatch_workgroups(1, 1, 1);

        Ok(())
    }
}