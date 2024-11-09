use std::ops::Deref;
use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::pbr::{MeshPipelineViewLayoutKey, MeshPipelineViewLayouts};
use bevy::prelude::{DirectAssetAccessExt, FromWorld, Msaa, Resource, World};
use bevy::render::render_resource::{BindGroupLayout, BindGroupLayoutEntries, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, MultisampleState, PipelineCache, PrimitiveState, RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, TextureFormat, TextureSampleType};
use bevy::render::render_resource::binding_types::{sampler, texture_2d, texture_depth_2d, uniform_buffer};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;
use bevy::render::view::{ViewDepthTexture, ViewUniforms};
use crate::systems::post_processing::settings::PostProcessSettings;
use bevy::render::view::ViewUniform;

// This contains global data used by the render pipeline. This will be created once on startup.
#[derive(Resource)]
pub struct PostProcessPipeline {
    pub layout: BindGroupLayout,
    pub sampler: Sampler,
    pub pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        // We need to define the bind group layout used for our pipeline
        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    // The depth texture
                    texture_depth_2d(),
                    // The sampler that will be used to sample the screen texture
                    sampler(SamplerBindingType::Filtering),
                    // The settings uniform that will control the effect
                    uniform_buffer::<PostProcessSettings>(true),
                    uniform_buffer::<ViewUniform>(true),
                ),
            ),
        );

        // We can create the sampler here since it won't change at runtime and doesn't depend on the view
        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        // Get the shader handle
        let shader = world.load_asset("shaders/post_processing.wgsl");

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("post_process_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}