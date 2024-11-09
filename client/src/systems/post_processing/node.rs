use bevy::ecs::query::QueryItem;
use bevy::ecs::system::lifetimeless::Read;
use bevy::pbr::{MeshViewBindGroup, ViewFogUniformOffset, ViewLightProbesUniformOffset, ViewLightsUniformOffset, ViewScreenSpaceReflectionsUniformOffset};
use bevy::prelude::{Camera3d, Query, World};
use bevy::render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy::render::render_graph::{NodeRunError, RenderGraphContext, ViewNode};
use bevy::render::render_resource::{BindGroupEntries, BindingResource, Operations, PipelineCache, RenderPassColorAttachment, RenderPassDescriptor, TextureUsages, TextureViewDescriptor};
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ViewDepthTexture, ViewTarget, ViewUniformOffset, ViewUniforms};
use crate::systems::post_processing::pipeline::PostProcessPipeline;
use crate::systems::post_processing::settings::PostProcessSettings;

// The post process node used for the render graph
#[derive(Default)]
pub struct PostProcessNode;

// The ViewNode trait is required by the ViewNodeRunner
impl ViewNode for PostProcessNode {
    // The node needs a query to gather data from the ECS in order to do its rendering,
    // but it's not a normal system so we need to define it manually.
    //
    // This query will only run on the view entity
    type ViewQuery = (
        &'static ViewTarget,
        // This makes sure the node only runs on cameras with the PostProcessSettings component
        &'static PostProcessSettings,
        // As there could be multiple post processing components sent to the GPU (one per camera),
        // we need to get the index of the one that is associated with the current view.
        &'static DynamicUniformIndex<PostProcessSettings>,
        // As there could be multiple post processing components sent to the GPU (one per camera),
        // we need to get the index of the one that is associated with the current view.
        Read<ViewDepthTexture>,
        &'static ViewUniformOffset,
    );

    // Runs the node logic
    // This is where you encode draw commands.
    //
    // This will run on every view on which the graph is running.
    // If you don't want your effect to run on every camera,
    // you'll need to make sure you have a marker component as part of [`ViewQuery`]
    // to identify which camera(s) should run the effect.
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (
            view_target,
            _post_process_settings,
            settings_index,
            depth_texture,
            view_uniform_offset
        ): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        // Get the pipeline resource that contains the global data we need
        // to create the render pipeline
        let post_process_pipeline = world.resource::<PostProcessPipeline>();

        let view_uniforms = world.resource::<ViewUniforms>().uniforms.binding().unwrap();

        // The pipeline cache is a cache of all previously created pipelines.
        // It is required to avoid creating a new pipeline each frame,
        // which is expensive due to shader compilation.
        let pipeline_cache = world.resource::<PipelineCache>();

        // Get the pipeline from the cache
        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        // Get the settings uniform binding
        let settings_uniforms = world.resource::<ComponentUniforms<PostProcessSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        // This will start a new "post process write", obtaining two texture
        // views from the view target - a `source` and a `destination`.
        // `source` is the "current" main texture and you _must_ write into
        // `destination` because calling `post_process_write()` on the
        // [`ViewTarget`] will internally flip the [`ViewTarget`]'s main
        // texture to the `destination` texture. Failing to do so will cause
        // the current main texture information to be lost.
        let post_process = view_target.post_process_write();

        // The bind_group gets created each frame.
        //
        // Normally, you would create a bind_group in the Queue set,
        // but this doesn't work with the post_process_write().
        // The reason it doesn't work is because each post_process_write will alternate the source/destination.
        // The only way to have the correct source/destination for the bind_group
        // is to make sure you get it during the node execution.
        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &post_process_pipeline.layout,
            // It's important for this to match the BindGroupLayout defined in the PostProcessPipeline
            &BindGroupEntries::sequential((
                // Make sure to use the source view
                post_process.source,
                BindingResource::TextureView(depth_texture.view()),
                // Use the sampler created for the pipeline
                &post_process_pipeline.sampler,
                // Set the settings binding
                settings_binding.clone(),
                view_uniforms
            )),
        );

        // Begin the render pass
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                // We need to specify the post process destination view here
                // to make sure we write to the appropriate texture.
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // This is mostly just wgpu boilerplate for drawing a fullscreen triangle,
        // using the pipeline/bind_group created above
        render_pass.set_render_pipeline(pipeline);
        // By passing in the index of the post process settings on this view, we ensure
        // that in the event that multiple settings were sent to the GPU (as would be the
        // case with multiple cameras), we use the correct one.
        render_pass.set_bind_group(0, &bind_group, &[settings_index.index(), view_uniform_offset.offset]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

/// A system that marks all view depth textures as readable in shaders.
///
/// The post processing pass needs to do this, and it doesn't happen by
/// default.
pub fn prepare_view_depth_textures(
    mut view_targets: Query<&mut Camera3d>
) {
    for mut camera in view_targets.iter_mut() {
        camera.depth_texture_usages.0 |= TextureUsages::TEXTURE_BINDING.bits();
    }
}