use bevy::pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline};
use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexBufferLayoutRef, VertexAttributeDescriptor};
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderDefVal, ShaderRef, SpecializedMeshPipelineError};
use crate::systems::chunk::builder::ATTRIBUTE_LIGHTING_COLOR;

pub type TranslucentChunkMaterial = ExtendedMaterial<StandardMaterial, TranslucentChunkMaterialExtension>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct TranslucentChunkMaterialExtension {
    #[uniform(100)]
    pub time: f32,
}

impl MaterialExtension for TranslucentChunkMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/extended_material_prepass.wgsl".into()
    }

    fn specialize(
        pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: MaterialExtensionKey<Self>
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.shader_defs
            .push("IS_TRANSLUCENT".into());
        Ok(())
    }
}