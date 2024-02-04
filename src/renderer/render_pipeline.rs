use bevy_ecs::system::Resource;

#[derive(Resource)]
struct TexturePipeline {
    pipeline: wgpu::RenderPipeline,
}
