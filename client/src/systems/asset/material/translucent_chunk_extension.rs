use bevy::pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline};
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError};
use crate::systems::asset::material::chunk_extension::add_vertex_extension;
use crate::systems::chunk::builder::{ATTRIBUTE_LIGHTING_COLOR, ATTRIBUTE_WIND_STRENGTH};

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
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>
    ) -> Result<(), SpecializedMeshPipelineError> {
        add_vertex_extension(layout, descriptor, ATTRIBUTE_LIGHTING_COLOR, 14);
        add_vertex_extension(layout, descriptor, ATTRIBUTE_WIND_STRENGTH, 15);

        descriptor.vertex.shader_defs
            .push("IS_TRANSLUCENT".into());
        descriptor.fragment.as_mut().unwrap().shader_defs
            .push("IS_TRANSLUCENT".into());
        Ok(())
    }
}